// SPDX-FileCopyrightText: 2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::{
    ecs::entity::EntityHashSet,
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
        let width = self.width.apply_if(reference, reference_size.x);
        let height = self.height.apply_if(reference, reference_size.y);
        self.finalize(target_size, width, height)
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

    fn finalize(&self, target_size: Vec2, width: Option<f32>, height: Option<f32>) -> Vec2 {
        if width.or(height).is_some() {
            self.apply_aspect_ratio(Vec2::new(
                width.unwrap_or(target_size.x),
                height.unwrap_or(target_size.y),
            ))
        } else {
            target_size
        }
    }

    fn intersect(
        &self,
        reference: &EntityResized,
        target: Entity,
        target_size: Vec2,
        parent_q: &Query<&Parent>,
        bounding_box_q: &Query<&BoundingBox>,
        dependencies: &mut EntityHashSet,
    ) -> Vec2 {
        let width = self.width.intersect(
            reference,
            target,
            parent_q,
            bounding_box_q,
            dependencies,
            |v| v.x,
        );
        let height = self.height.intersect(
            reference,
            target,
            parent_q,
            bounding_box_q,
            dependencies,
            |v| v.y,
        );
        self.finalize(target_size, width, height)
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
        dependencies: &mut EntityHashSet,
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
                            .map(|v| {
                                dependencies.insert(reference.entity);
                                dependencies.insert(other);
                                v
                            })
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
    pub dependencies: EntityHashSet,
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
            if matches!(resizable.width.reference, Reference::Window)
                || matches!(resizable.height.reference, Reference::Window)
            {
                debug!(
                    "Window resize for {entity:?}: {old_size:?} != {size:?}, reference = {:?}",
                    event.size
                );
                entity_ev.send(EntityResized {
                    entity,
                    size,
                    dependencies: Default::default(),
                });
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
                    debug!(
                        "Parent resize for {child:?}: {old_size:?} != {size:?}, reference = {:?}",
                        parent.size
                    );
                    new_events.push(EntityResized {
                        entity: child,
                        size,
                        dependencies: EntityHashSet::from_iter([parent.entity]),
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
            let mut dependencies = EntityHashSet::default();
            let old_size = bounding_box.size();
            let size = resizable.intersect(
                reference,
                entity,
                old_size,
                &parent_q,
                &bounding_box_q,
                &mut dependencies,
            );
            if old_size != size {
                debug!("Intersection resize for {entity:?}: {old_size:?} != {size:?}, reference = {:?}", reference.size);
                new_events.push(EntityResized {
                    entity,
                    size,
                    dependencies,
                })
            }
        }
    }

    param_set.p1().send_batch(new_events);
}

fn entity_resize(
    mut events: EventReader<EntityResized>,
    mut resize_q: Query<&mut BoundingBox, With<Resizable>>,
) {
    let mut count = 0;
    let mut resized = EntityHashSet::default();
    let mut all_dependencies = EntityHashSet::default();
    let mut out_of_order = EntityHashSet::default();
    let mut already_resized = 0;
    for event in events.read() {
        count += 1;
        if !resized.insert(event.entity) {
            already_resized += 1;
        }
        if let Ok(mut bounding_box) = resize_q.get_mut(event.entity) {
            bounding_box.resize(event.size);
        }
        out_of_order.extend(event.dependencies.difference(&resized));
        all_dependencies.extend(&event.dependencies);
        // } else {
        //     let dependencies = event.dependencies.len();
        //     let have_been_resized = event.dependencies.intersection(&set).count();
        //     debug!("Entity already in resize set. Depended on {dependencies:?} other entities, of which {have_been_resized:?} were resized.");
        // }
    }
    let were_resized = resized.len();
    let were_deps = resized.intersection(&all_dependencies).count();
    let were_ooo = resized.intersection(&out_of_order).count();
    if were_resized > 0 {
        debug!("Received {count:?} resize requests. Resized {were_resized:?} entities of which {were_deps:?} were dependencies, {were_ooo:?} were resized out of order, and {already_resized:?} were already resized");
    }
}
