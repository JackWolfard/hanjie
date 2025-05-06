// SPDX-FileCopyrightText: 2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use std::cell::RefCell;

use bevy::{
    ecs::entity::{EntityHashMap, EntityHashSet},
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

struct ResizeTarget<'a> {
    entity: Entity,
    bounding_box: &'a BoundingBox,
}

trait ResizeEvent {}

impl Resizable {
    fn try_resize(&self, target: ResizeTarget, event: &impl ResizeEvent) -> Option<EntityResized> {
        // self.constrain_to_aspect_ratio(
        // self.width.resize(reference, reference.width()),
        // self.height.resize(reference, reference.height()),
        // )
        None
    }

    fn apply_if(&self, reference: Reference, reference_size: Vec2, target_size: Vec2) -> Vec2 {
        let width = self.width.apply_if(reference, reference_size.x);
        let height = self.height.apply_if(reference, reference_size.y);
        self.finalize(target_size, width, height)
    }

    fn constrain_to_aspect_ratio(&self, width: f32, height: f32) -> Vec2 {
        self.aspect_ratio
            .and_then(|aspect_ratio| {
                let new_aspect_ratio: f32 = AspectRatio::new(width, height).into();
                match new_aspect_ratio.total_cmp(&aspect_ratio) {
                    std::cmp::Ordering::Less => Some(Vec2::new(width, width / aspect_ratio)),
                    std::cmp::Ordering::Equal => None,
                    std::cmp::Ordering::Greater => Some(Vec2::new(height * aspect_ratio, height)),
                }
            })
            .unwrap_or(Vec2::new(width, height))
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
    fn try_resize(&self) -> Option<f32> {
        None
    }

    fn resize(&self, reference: f32) -> f32 {
        self.constraint.apply(reference)
    }

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

    fn _intersect(&self, x: Entity, y: Entity) {}
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

#[derive(Debug, Clone, Copy)]
enum KnownReference {
    Intersect(Vec2, Vec2),
    Parent(Vec2),
    Window(Vec2),
}

impl KnownReference {
    fn size(&self) -> Vec2 {
        match self {
            KnownReference::Intersect(a, b) => (*a - *b).abs(),
            KnownReference::Parent(a) => *a,
            KnownReference::Window(a) => *a,
        }
    }

    fn width(&self) -> f32 {
        self.size().x
    }

    fn height(&self) -> f32 {
        self.size().y
    }
}

impl EntityReference {
    fn entity(&self, target: Entity, parent_q: &Query<&Parent>) -> Option<Entity> {
        let mut ancestors = parent_q.iter_ancestors(target);
        match self {
            EntityReference::Grandparent => ancestors.nth(1),
            EntityReference::Parent => ancestors.next(),
        }
    }

    fn size(
        &self,
        target: Entity,
        parent_q: &Query<&Parent>,
        bounding_box_q: &Query<&BoundingBox>,
    ) -> Option<Vec2> {
        self.entity(target, parent_q)
            .and_then(|e| bounding_box_q.get(e).ok())
            .map(|b| b.size())
    }
}

#[derive(Event, Debug)]
pub struct WindowResized {
    pub size: Vec2,
}

impl ResizeEvent for WindowResized {}

#[derive(Event, Debug)]
pub struct EntityResized {
    pub entity: Entity,
    pub size: Vec2,
    pub dependencies: EntityHashSet,
}

impl ResizeEvent for EntityResized {}

#[derive(Event, Debug)]
pub struct EventualEntityResized {
    pub entity: Entity,
}

fn engine_window_resize(
    mut engine_ev: EventReader<window::WindowResized>,
    window_q: Query<Entity, With<PrimaryWindow>>,
    mut window_ev: EventWriter<WindowResized>,
) {
    if let Ok(primary_window) = window_q.get_single() {
        if let Some(event) = engine_ev
            .read()
            .filter(|event| event.window == primary_window)
            .last()
        {
            window_ev.send(WindowResized {
                size: Vec2::new(event.width, event.height),
            });
        }
    }
}

fn window_resize(
    mut window_ev: EventReader<WindowResized>,
    resize_q: Query<(Entity, &BoundingBox, &Resizable)>,
    mut entity_ev: EventWriter<EntityResized>,
) {
    if let Some(window) = window_ev.read().last() {
        entity_ev.send_batch(
            resize_q
                .iter()
                .filter_map(|(entity, bounding_box, resizable)| {
                    resizable.try_resize(
                        ResizeTarget {
                            entity,
                            bounding_box,
                        },
                        window,
                    )
                }),
        );
    }
}

fn _entity_resize(mut entity_ev: EventReader<EntityResized>) {
    let unresolved_entities: EntityHashMap<Vec2> = entity_ev
        .read()
        .map(|event| (event.entity, event.size))
        .collect();
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

// for an entity
// 1. if all deps are not in resized_entities, then use calculated size
// 2. if at least one dep is in resized_entities, then recalculate new size
//     a. that dep / those deps, must all have finalized sizes
// 3. multiple resize events for an entity is valid, but this might
//    indicate an entity's deps were also resized in this batch. Probably a
//    recalculate. Can result in error if invariant of entity always having
//    same deps is violated. For instance, event 1 being c = intersect(a, b)
//    & event 2 being c = intersect(a, d) is invalid since deps are
//    different between two events
fn eventual_entity_resize(mut events: EventReader<EntityResized>) {
    let mut unresolved_entities = EntityHashMap::<(Vec2, EntityHashSet)>::default();
    for event in events.read() {
        unresolved_entities.insert(event.entity, (event.size, event.dependencies.clone()));
    }

    let resized_entities = EntityHashSet::from_iter(unresolved_entities.keys().cloned());

    unresolved_entities
        .iter_mut()
        .for_each(|(_, (_, dependencies))| dependencies.retain(|e| resized_entities.contains(e)));

    let mut finalized_entities = EntityHashMap::<Vec2>::default();

    unresolved_entities
        .extract_if(|_, (_, dependencies)| dependencies.is_empty())
        .for_each(|(entity, (size, _))| {
            finalized_entities.insert(entity, size);
        });

    let finalized_cell = RefCell::new(finalized_entities);
    while !unresolved_entities.is_empty() {
        unresolved_entities
            .extract_if(|_, (_, dependencies)| {
                dependencies
                    .iter()
                    .all(|d| finalized_cell.try_borrow().is_ok_and(|h| h.contains_key(d)))
            })
            .for_each(|(entity, _)| {
                // recalculate entity size based on dependencies finalized values
                finalized_cell.borrow_mut().insert(entity, Vec2::splat(0.0));
            });
    }
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
