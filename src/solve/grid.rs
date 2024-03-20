// SPDX-FileCopyrightText: 2023-2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::{math::AspectRatio, prelude::*};

use crate::{
    layout::{
        bounding_box::BoundingBox,
        position::{self, Alignable},
        size::{self, Resizable},
    },
    puzzle::{GridSize, Puzzle},
    solve::{cell::CellBundle, constraint},
};

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Component)]
pub struct Grid;

#[derive(Bundle)]
pub struct GridBundle {
    #[bundle()]
    spatial_bundle: SpatialBundle,
    bounding_box: BoundingBox,
    alignable: Alignable,
    resizable: Resizable,
    size: GridSize,
}

impl GridBundle {
    pub fn new(puzzle: &Puzzle) -> GridBundle {
        let grid_size = GridSize {
            columns: puzzle.grid.columns,
            rows: puzzle.grid.rows,
        };
        GridBundle {
            spatial_bundle: SpatialBundle::default(),
            bounding_box: BoundingBox::init(Color::LIME_GREEN),
            alignable: Alignable::new(
                position::Reference::Parent,
                position::Alignment::Standard(
                    position::VerticalAlign::Bottom,
                    position::HorizontalAlign::Right,
                ),
            ),
            resizable: Resizable::new(
                size::ResizableField {
                    constraint: size::Constraint::Pct(0.6),
                    reference: size::Reference::Parent,
                },
                size::ResizableField {
                    constraint: size::Constraint::Pct(0.6),
                    reference: size::Reference::Parent,
                },
                Some(AspectRatio::new(
                    grid_size.columns as f32,
                    grid_size.rows as f32,
                )),
            ),
            size: grid_size,
        }
    }
}

pub fn spawn(builder: &mut ChildBuilder, puzzle: &Puzzle) {
    builder
        .spawn((GridBundle::new(puzzle), Grid))
        .with_children(|parent| {
            (0..puzzle.grid.columns).for_each(|column| {
                (0..puzzle.grid.rows).for_each(|row| {
                    parent.spawn(CellBundle::new(row, column, &puzzle.grid));
                });
            });
            for constraint in puzzle.constraints.iter() {
                constraint::spawn(parent, constraint.clone(), &puzzle.grid);
            }
        });
}
