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
                    parent_resize,
                    intersect_resize,
                    entity_resize,
                )
                    .chain()
                    .in_set(LayoutSet::Size),
            );
    }
}

#[derive(Component)]
pub struct Resizable {
    pub width: ResizableField,
    pub height: ResizableField,
    aspect_ratio: Option<f32>,
}

impl Resizable {
    pub fn new(
        width: ResizableField,
        height: ResizableField,
        aspect_ratio: Option<AspectRatio>,
    ) -> Self {
        Self {
            width,
            height,
            aspect_ratio: aspect_ratio.map(|ar| ar.into()),
        }
    }
}

impl Resizable {
    fn apply_if(&self, reference: Reference, reference_size: Vec2, target_size: Vec2) -> Vec2 {
        let width = self
            .width
            .apply_if(reference, reference_size.x)
            .unwrap_or(target_size.x);
        let height = self
            .height
            .apply_if(reference, reference_size.y)
            .unwrap_or(target_size.y);
        self.apply_aspect_ratio(Vec2::new(width, height))
    }

    fn apply_aspect_ratio(&self, size: Vec2) -> Vec2 {
        if let Some(aspect_ratio) = self.aspect_ratio {
            let mut width = size.x;
            let mut height = size.y;
            let new_aspect_ratio: f32 = AspectRatio::new(width, height).into();
            if new_aspect_ratio < aspect_ratio {
                height = width / aspect_ratio;
            } else if new_aspect_ratio > aspect_ratio {
                width = height * aspect_ratio;
            }
            Vec2::new(width, height)
        } else {
            size
        }
    }

    fn intersect(
        &self,
        reference: &EntityResized,
        target: Entity,
        target_size: Vec2,
        parent_q: &Query<&Parent>,
        bounding_box_q: &Query<&BoundingBox>,
    ) -> Vec2 {
        let width = self
            .width
            .intersect(reference, target, parent_q, bounding_box_q, |v| v.x)
            .unwrap_or(target_size.x);
        let height = self
            .height
            .intersect(reference, target, parent_q, bounding_box_q, |v| v.y)
            .unwrap_or(target_size.y);
        self.apply_aspect_ratio(Vec2::new(width, height))
    }
}

pub struct ResizableField {
    pub constraint: Constraint,
    pub reference: Reference,
}

impl ResizableField {
    fn apply_if(&self, reference: Reference, size: f32) -> Option<f32> {
        if self.reference == reference {
            Some(self.constraint.apply(size))
        } else {
            None
        }
    }

    fn intersect(
        &self,
        reference: &EntityResized,
        target: Entity,
        parent_q: &Query<&Parent>,
        bounding_box_q: &Query<&BoundingBox>,
        dim: fn(Vec2) -> f32,
    ) -> Option<f32> {
        match self.reference {
            Reference::Intersect(alpha, beta) => {
                let alpha = alpha.entity(target, parent_q);
                let beta = beta.entity(target, parent_q);

                match (alpha, beta) {
                    (Some(alpha), Some(beta)) => {
                        let other = match (reference.entity == alpha, reference.entity == beta) {
                            (true, true) => {
                                panic!("Expected intersect to not be between same entity")
                            }
                            (true, false) => beta,
                            (false, true) => alpha,
                            (false, false) => return None,
                        };
                        bounding_box_q
                            .get(other)
                            .ok()
                            .map(|b| (reference.size - b.size()).abs())
                            .map(dim)
                    }
                    _ => panic!("Expected to find entity references"),
                }
            }
            _ => None,
        }
    }
}

pub enum Constraint {
    Fill,
    Fr(i32),
    Pct(f32),
}

impl Constraint {
    pub fn apply(&self, size: f32) -> f32 {
        match self {
            Constraint::Fr(fr) => size * (1.0 / *fr as f32),
            Constraint::Pct(pct) => size * pct,
            Constraint::Fill => size,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Reference {
    Intersect(EntityReference, EntityReference),
    Parent,
    Window,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EntityReference {
    Grandparent,
    Parent,
}

impl EntityReference {
    fn entity(&self, target: Entity, parent_q: &Query<&Parent>) -> Option<Entity> {
        let mut ancestors = parent_q.iter_ancestors(target);
        match self {
            EntityReference::Grandparent => ancestors.nth(1),
            EntityReference::Parent => ancestors.next(),
        }
    }
}

#[derive(Event, Debug)]
pub struct WindowResized {
    pub size: Vec2,
}

#[derive(Event, Debug)]
pub struct EntityResized {
    pub entity: Entity,
    pub size: Vec2,
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
    resize_q: Query<(Entity, &BoundingBox, &Resizable)>,
    mut entity_ev: EventWriter<EntityResized>,
) {
    for event in window_ev.read() {
        for (entity, bounding_box, resizable) in &resize_q {
            let old_size = bounding_box.size();
            let size = resizable.apply_if(Reference::Window, event.size, old_size);
            if old_size != size {
                entity_ev.send(EntityResized { entity, size });
            }
        }
    }
}

fn parent_resize(
    mut param_set: ParamSet<(EventReader<EntityResized>, EventWriter<EntityResized>)>,
    resize_q: Query<(Entity, &Parent, &BoundingBox, &Resizable)>,
) {
    let mut new_events = Vec::<EntityResized>::new();
    for parent in param_set.p0().read() {
        for (child, parent_entity, bounding_box, resizable) in resize_q.iter() {
            if parent.entity == parent_entity.get() {
                let old_size = bounding_box.size();
                let size = resizable.apply_if(Reference::Parent, parent.size, old_size);
                if old_size != size {
                    new_events.push(EntityResized {
                        entity: child,
                        size,
                    })
                }
            }
        }
    }

    param_set.p1().send_batch(new_events);
}

fn intersect_resize(
    mut param_set: ParamSet<(EventReader<EntityResized>, EventWriter<EntityResized>)>,
    resize_q: Query<(Entity, &BoundingBox, &Resizable)>,
    parent_q: Query<&Parent>,
    bounding_box_q: Query<&BoundingBox>,
) {
    let mut new_events = Vec::<EntityResized>::new();
    for reference in param_set.p0().read() {
        for (entity, bounding_box, resizable) in &resize_q {
            let old_size = bounding_box.size();
            let size = resizable.intersect(reference, entity, old_size, &parent_q, &bounding_box_q);
            if old_size != size {
                new_events.push(EntityResized { entity, size })
            }
        }
    }

    param_set.p1().send_batch(new_events);
}

fn entity_resize(
    mut events: EventReader<EntityResized>,
    mut resize_q: Query<&mut BoundingBox, With<Resizable>>,
) {
    for event in events.read() {
        if let Ok(mut bounding_box) = resize_q.get_mut(event.entity) {
            bounding_box.resize(event.size);
        }
    }
}
