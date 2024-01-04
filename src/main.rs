// SPDX-FileCopyrightText: 2023-2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
    window::WindowTheme,
};

use crate::{
    action::ActionPlugin, camera::CameraPlugin, cell::CellPlugin, grid::GridPlugin,
    input::InputPlugin,
};

mod action;
mod camera;
mod cell;
mod grid;
mod input;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "hanjie".to_string(),
                    window_theme: Some(WindowTheme::Dark),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(LogPlugin {
                filter: "info,wgpu_core=warn,wgpu_hal=warn,hanjie=debug".into(),
                level: Level::DEBUG,
            }),))
        .add_plugins(ActionPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(CellPlugin)
        .add_plugins(GridPlugin)
        .add_plugins(InputPlugin)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
