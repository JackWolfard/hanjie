// SPDX-FileCopyrightText: 2023-2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::prelude::*;

use crate::{
    puzzle::{GridSize, Puzzle},
    solve::cell::{CellBundle, CELL_GUTTER, CELL_SIZE},
};

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Component)]
pub struct Grid;

#[derive(Bundle)]
pub struct GridBundle {
    size: GridSize,

    #[bundle()]
    spatial_bundle: SpatialBundle,
}

impl GridBundle {
    pub fn new(puzzle: &Puzzle) -> GridBundle {
        GridBundle {
            size: GridSize {
                columns: puzzle.grid.columns,
                rows: puzzle.grid.rows,
            },
            spatial_bundle: SpatialBundle {
                transform: center_grid(&puzzle.grid),
                ..default()
            },
        }
    }
}

fn center_grid(grid: &GridSize) -> Transform {
    let width = grid.columns as f32;
    let width = width * CELL_SIZE + (width - 1.0) * CELL_GUTTER;
    let height = grid.rows as f32;
    let height = height * CELL_SIZE + (height - 1.0) * CELL_GUTTER;
    Transform::from_xyz(-(width / 2.0), -(height / 2.0), 0.0)
}

pub fn spawn(builder: &mut ChildBuilder, puzzle: &Puzzle) {
    builder
        .spawn((GridBundle::new(puzzle), Grid))
        .with_children(|parent| {
            (0..puzzle.grid.columns).for_each(|column| {
                (0..puzzle.grid.rows).for_each(|row| {
                    parent.spawn(CellBundle::new(column, row));
                });
            });
        });
}
