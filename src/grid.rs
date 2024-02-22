// SPDX-FileCopyrightText: 2023-2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::prelude::*;

use crate::{app::AppState, cell::CellBundle, schedule::InGameSet};

const GRID_ROWS: i32 = 5;
const GRID_COLUMNS: i32 = 5;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::InGame),
            spawn_grid.in_set(InGameSet::OnEnter),
        )
        .add_systems(
            OnEnter(AppState::InGame),
            spawn_cells.in_set(InGameSet::PostOnEnter),
        )
        .add_systems(
            OnExit(AppState::InGame),
            despawn_grid.in_set(InGameSet::OnExit),
        );
    }
}

#[derive(Component)]
struct GridSize {
    columns: i32,
    rows: i32,
}

#[derive(Component)]
struct Grid;

#[derive(Bundle)]
struct GridBundle {
    size: GridSize,

    #[bundle()]
    spatial_bundle: SpatialBundle,
}

impl GridBundle {
    fn new(columns: i32, rows: i32) -> GridBundle {
        GridBundle {
            size: GridSize { columns, rows },
            spatial_bundle: SpatialBundle::default(),
        }
    }
}

fn spawn_grid(mut commands: Commands) {
    commands.spawn((GridBundle::new(GRID_COLUMNS, GRID_ROWS), Grid));
}

fn spawn_cells(mut commands: Commands, query: Query<(Entity, &GridSize), With<Grid>>) {
    let (e, size) = query.single();
    commands.entity(e).with_children(|parent| {
        (0..size.columns).for_each(|column| {
            (0..size.rows).for_each(|row| {
                parent.spawn(CellBundle::new(column, row));
            });
        });
    });
}

fn despawn_grid(mut commands: Commands, query: Query<Entity, With<Grid>>) {
    let e = query.single();
    commands.entity(e).despawn_recursive();
}
