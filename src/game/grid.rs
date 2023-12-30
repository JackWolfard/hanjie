// SPDX-FileCopyrightText: Copyright (c) 2023 Jack Wolfard
// SPDX-License-Identifier: Apache-2.0

use bevy::prelude::*;

use crate::{
    constant::{GRID_COLUMNS, GRID_ROWS},
    game::cell::CellBundle,
};

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, presetup)
            .add_systems(Startup, setup);
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

fn presetup(mut commands: Commands) {
    commands.spawn((GridBundle::new(GRID_COLUMNS, GRID_ROWS), Grid));
}

fn setup(mut commands: Commands, query: Query<(Entity, &GridSize), With<Grid>>) {
    let (e, size) = query.single();
    commands.entity(e).with_children(|parent| {
        (0..size.columns).for_each(|column| {
            (0..size.rows).for_each(|row| {
                parent.spawn(CellBundle::new(column, row));
            });
        });
    });
}
