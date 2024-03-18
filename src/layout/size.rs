// SPDX-FileCopyrightText: 2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::{
    math::AspectRatio,
    prelude::*,
    window::{self, PrimaryWindow},
};

use crate::layout::{bounding_box::BoundingBox, schedule::LayoutSet};

pub struct SizePlugin;

impl Plugin for SizePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EntityResized>()
            .add_event::<WindowResized>()
            .add_systems(
                Update,
                (
                    engine_window_resize,
                    window_resize,
                    (entity_resize, child_resize),
                )
                    .chain()
                    .in_set(LayoutSet::Size),
            );
    }
}

#[derive(Component)]
pub struct Resizable {
    pub reference: Reference,
    pub constraints: Constraints,
}

impl Resizable {
    pub fn new(
        reference: Reference,
        width: Constraint,
        height: Constraint,
        aspect_ratio: Option<AspectRatio>,
    ) -> Self {
        Self {
            reference,
            constraints: Constraints {
                width,
                height,
                aspect_ratio: aspect_ratio.map(|ar| ar.into()),
            },
        }
    }
}

pub enum Reference {
    Parent,
    Window,
}

pub struct Constraints {
    pub height: Constraint,
    pub width: Constraint,
    pub aspect_ratio: Option<f32>,
}

impl Constraints {
    pub fn apply(&self, size: Vec2) -> Vec2 {
        let mut width = self.width.apply(size.x);
        let mut height = self.height.apply(size.y);
        if let Some(aspect_ratio) = self.aspect_ratio {
            let new_aspect_ratio: f32 = AspectRatio::new(width, height).into();
            if new_aspect_ratio < aspect_ratio {
                height = width / aspect_ratio;
            } else if new_aspect_ratio > aspect_ratio {
                width = height * aspect_ratio;
            }
        }
        Vec2::new(width, height)
    }
}

pub enum Constraint {
    Pct(f32),
}

impl Constraint {
    pub fn apply(&self, size: f32) -> f32 {
        match self {
            Constraint::Pct(pct) => size * pct,
        }
    }
}

#[derive(Event, Debug)]
struct WindowResized {
    size: Vec2,
}

#[derive(Event, Debug)]
pub(super) struct EntityResized {
    pub(super) entity: Entity,
    pub(super) size: Vec2,
}

fn engine_window_resize(
    mut ev_engine: EventReader<window::WindowResized>,
    window_q: Query<Entity, With<PrimaryWindow>>,
    mut ev_window: EventWriter<WindowResized>,
) {
    for event in ev_engine.read() {
        for window in window_q.iter() {
            if window == event.window {
                ev_window.send(WindowResized {
                    size: Vec2::new(event.width, event.height),
                });
            }
        }
    }
}

fn window_resize(
    mut window_ev: EventReader<WindowResized>,
    mut resize_q: Query<(Entity, &Resizable)>,
    mut entity_ev: EventWriter<EntityResized>,
) {
    for event in window_ev.read() {
        for (entity, constraints) in resize_q.iter_mut().filter_map(|(e, r)| match r.reference {
            Reference::Window => Some((e, &r.constraints)),
            _ => None,
        }) {
            entity_ev.send(EntityResized {
                entity,
                size: constraints.apply(event.size),
            });
        }
    }
}

fn child_resize(
    mut param_set: ParamSet<(EventReader<EntityResized>, EventWriter<EntityResized>)>,
    resize_q: Query<(Entity, &Parent, &Resizable)>,
) {
    let mut new_events = Vec::<EntityResized>::new();
    for EntityResized {
        entity: parent_entity,
        size,
    } in param_set.p0().read()
    {
        for (child, parent, resizable) in resize_q.iter() {
            if let Reference::Parent = resizable.reference {
                if *parent_entity == parent.get() {
                    new_events.push(EntityResized {
                        entity: child,
                        size: resizable.constraints.apply(*size),
                    })
                }
            }
        }
    }

    param_set.p1().send_batch(new_events);
}

fn entity_resize(
    mut events: EventReader<EntityResized>,
    mut resize_q: Query<&mut BoundingBox, With<Resizable>>,
) {
    for EntityResized { entity, size } in events.read() {
        if let Ok(mut bounding_box) = resize_q.get_mut(*entity) {
            bounding_box.resize(*size);
        }
    }
}
