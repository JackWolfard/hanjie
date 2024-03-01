// SPDX-FileCopyrightText: 2023-2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};

use crate::{
    app::AppState,
    cell::{Cell, Location},
    schedule::InGameSet,
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LogPlugin {
            filter: "info,wgpu_core=warn,wgpu_hal=warn,hanjie=debug".into(),
            level: Level::DEBUG,
        })
        .add_systems(Update, hello_world)
        .add_systems(
            OnEnter(AppState::InGame),
            print_cell_location.after(InGameSet::PostOnEnter),
        );
    }
}

fn hello_world() {
    debug!("JACK!");
}

fn print_cell_location(query: Query<(&GlobalTransform, &Location), With<Cell>>) {
    for (transform, location) in query.iter() {
        let translation = transform.translation();
        debug!(
            "Cell({},{}) is located at ({},{})",
            location.column, location.row, translation.x, translation.y
        );
    }
}
