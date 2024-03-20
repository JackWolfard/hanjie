// SPDX-FileCopyrightText: 2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::{ecs::entity::EntityHashSet, prelude::*};

use crate::{
    layout::{bounding_box::BoundingBox, schedule::LayoutSet, size::EntityResized},
    puzzle::{GridSize, Line, Position},
};

pub struct PositionPlugin;

impl Plugin for PositionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EntityRealigned>().add_systems(
            Update,
            (child_realign, self_realign, realign).in_set(LayoutSet::Position),
        );
    }
}

#[derive(Event, Debug)]
pub struct EntityRealigned {
    pub entity: Entity,
    pub position: Vec2,
}

#[derive(Component)]
pub struct Alignable {
    pub reference: Reference,
    pub alignment: Alignment,
}

impl Alignable {
    pub fn new(reference: Reference, alignment: Alignment) -> Self {
        Self {
            reference,
            alignment,
        }
    }
}

pub enum Reference {
    Parent,
}

pub enum Alignment {
    Standard(VerticalAlign, HorizontalAlign),
    Grid(GridSize, Position, Option<Spacing>),
    Line(GridSize, Line),
}

impl Alignment {
    fn apply(&self, reference_size: Vec2, size: Vec2) -> Vec2 {
        match self {
            Alignment::Standard(vertical, horizontal) => {
                let x = Align::from(*horizontal).apply(reference_size.x, size.x);
                let y = Align::from(*vertical).apply(reference_size.y, size.y);
                Vec2::new(x, y)
            }
            Alignment::Grid(grid, position, spacing) => {
                let grid = Vec2::from(*grid);
                let position = Vec2::from(*position);

                let mut cell_spacing = reference_size / grid;

                if let Some(Spacing::Even) = spacing {
                    cell_spacing = Vec2::splat(cell_spacing.min_element());
                }

                let offset_if_even = ((grid + 1.0) % 2.0).floor() * 0.5;
                let start_offset = (grid / 2.0).floor() - offset_if_even;
                let start = cell_spacing * -start_offset;

                start + cell_spacing * position
            }
            Alignment::Line(grid, line) => {
                let line_alignment = |reference_size: f32, lines: f32, line: f32| -> f32 {
                    let spacing = reference_size / lines;
                    let offset_if_even = ((lines + 1.0) % 2.0).floor() * 0.5;
                    let start_offset = (lines / 2.0).floor() - offset_if_even;
                    let start = spacing * -start_offset;
                    start + spacing * line
                };
                let offset = (reference_size + size) / 2.0;
                match line {
                    Line::Column(column) => Vec2::new(
                        line_alignment(reference_size.x, grid.columns as f32, *column as f32),
                        offset.y,
                    ),
                    Line::Row(row) => Vec2::new(
                        -offset.x,
                        line_alignment(reference_size.y, grid.rows as f32, *row as f32),
                    ),
                }
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum Spacing {
    Even,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum Align {
    Start,
    Center,
    End,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum VerticalAlign {
    Top,
    Center,
    Bottom,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum HorizontalAlign {
    Left,
    Center,
    Right,
}

impl From<VerticalAlign> for Align {
    fn from(value: VerticalAlign) -> Self {
        match value {
            VerticalAlign::Top => Self::End,
            VerticalAlign::Center => Self::Center,
            VerticalAlign::Bottom => Self::Start,
        }
    }
}

impl From<HorizontalAlign> for Align {
    fn from(value: HorizontalAlign) -> Self {
        match value {
            HorizontalAlign::Left => Self::Start,
            HorizontalAlign::Center => Self::Center,
            HorizontalAlign::Right => Self::End,
        }
    }
}

impl Align {
    fn apply(&self, reference_size: f32, size: f32) -> f32 {
        match self {
            Align::Start => -reference_size / 2.0 + size / 2.0,
            Align::Center => 0.0,
            Align::End => reference_size / 2.0 - size / 2.0,
        }
    }
}

fn self_realign(
    mut size_ev: EventReader<EntityResized>,
    align_q: Query<(Entity, &Alignable, &Parent)>,
    bounding_box_q: Query<&BoundingBox>,
    mut align_ev: EventWriter<EntityRealigned>,
) {
    for resize in size_ev.read() {
        if let Ok((entity, alignable, parent)) = align_q.get(resize.entity) {
            if let Ok(parent_bounding_box) = bounding_box_q.get(parent.get()) {
                align_ev.send(EntityRealigned {
                    entity,
                    position: alignable
                        .alignment
                        .apply(parent_bounding_box.size(), resize.size),
                });
            }
        }
    }
}

fn child_realign(
    mut size_ev: EventReader<EntityResized>,
    parent_q: Query<(&BoundingBox, &Children)>,
    child_q: Query<(&Alignable, &BoundingBox), With<Parent>>,
    mut align_ev: EventWriter<EntityRealigned>,
) {
    for EntityResized { entity: parent, .. } in size_ev.read() {
        // get parent's size
        if let Ok((bounding_box, children)) = parent_q.get(*parent) {
            let parent_size = bounding_box.size();
            for child in children {
                if let Ok((
                    Alignable {
                        reference: Reference::Parent,
                        alignment,
                    },
                    bounding_box,
                )) = child_q.get(*child)
                {
                    let child_size = bounding_box.size();
                    align_ev.send(EntityRealigned {
                        entity: *child,
                        position: alignment.apply(parent_size, child_size),
                    });
                }
            }
        }
    }
}

fn realign(
    mut events: EventReader<EntityRealigned>,
    mut align_q: Query<&mut Transform, With<Alignable>>,
) {
    let mut set = EntityHashSet::default();
    for event in events.read() {
        if set.insert(event.entity) {
            if let Ok(mut transform) = align_q.get_mut(event.entity) {
                transform.translation = event.position.extend(transform.translation.z);
            }
        } else {
            debug!("Entity already in realign set");
        }
    }
}
