// SPDX-FileCopyrightText: 2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::prelude::*;

use crate::{
    layout::{bounding_box::BoundingBox, schedule::LayoutSet, size::EntityResized},
    puzzle::{GridSize, Line, Position},
};

pub struct PositionPlugin;

impl Plugin for PositionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EntityRealign>()
            .add_systems(Update, (child_realign, realign).in_set(LayoutSet::Position));
    }
}

#[derive(Event, Debug)]
struct EntityRealign {
    entity: Entity,
    position: Vec2,
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
    fn apply(&self, reference_center: Vec2, reference_size: Vec2, size: Vec2) -> Vec2 {
        match self {
            Alignment::Standard(vertical, horizontal) => {
                let x =
                    Align::from(*horizontal).apply(reference_center.x, reference_size.x, size.x);
                let y = Align::from(*vertical).apply(reference_center.y, reference_size.y, size.y);
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
    fn apply(&self, reference_center: f32, reference_size: f32, size: f32) -> f32 {
        match self {
            Align::Start => reference_center - reference_size / 2.0 + size / 2.0,
            Align::Center => reference_center,
            Align::End => reference_center + reference_size / 2.0 - size / 2.0,
        }
    }
}

fn child_realign(
    mut size_ev: EventReader<EntityResized>,
    parent_q: Query<(&Transform, &BoundingBox, &Children)>,
    child_q: Query<(&Alignable, &BoundingBox), With<Parent>>,
    mut align_ev: EventWriter<EntityRealign>,
) {
    for EntityResized { entity: parent, .. } in size_ev.read() {
        // get parent's center & size
        if let Ok((transform, bounding_box, children)) = parent_q.get(*parent) {
            let parent_center = transform.translation.xy();
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
                    align_ev.send(EntityRealign {
                        entity: *child,
                        position: alignment.apply(parent_center, parent_size, child_size),
                    });
                }
            }
        }

        //
    }
}

fn realign(
    mut events: EventReader<EntityRealign>,
    mut align_q: Query<&mut Transform, With<Alignable>>,
) {
    for EntityRealign { entity, position } in events.read() {
        if let Ok(mut transform) = align_q.get_mut(*entity) {
            transform.translation = position.extend(transform.translation.z);
        }
    }
}
