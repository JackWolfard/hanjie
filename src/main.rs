// SPDX-FileCopyrightText: 2023-2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
    window::WindowTheme,
};

use crate::{constant::BACKGROUND_COLOR, game::GamePlugin};

mod constant;
mod game;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
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
                }),
            GamePlugin,
        ))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
