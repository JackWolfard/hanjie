// SPDX-FileCopyrightText: 2023-2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::{log::LogPlugin, prelude::*, window::WindowTheme};

use crate::{
    action::ActionPlugin,
    app::AppPlugin,
    camera::CameraPlugin,
    feature::FeaturePlugins,
    input::InputPlugin,
    layout::LayoutPlugins,
    puzzle::PuzzlePlugin,
    schedule::SchedulePlugin,
    solve::SolvePlugin,
    ui::{solve::SolveUiPlugin, UiPlugins},
};

mod action;
mod app;
mod camera;
mod debug;
mod feature;
mod input;
mod inspect;
mod layout;
mod puzzle;
mod schedule;
mod solve;
mod ui;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "hanjie".to_string(),
                        window_theme: Some(WindowTheme::Dark),
                        ..default()
                    }),
                    ..default()
                })
                .disable::<LogPlugin>(),
        )
        .add_plugins(ActionPlugin)
        .add_plugins(AppPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(FeaturePlugins)
        .add_plugins(InputPlugin)
        .add_plugins(LayoutPlugins)
        .add_plugins(PuzzlePlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(SolvePlugin)
        .add_plugins(UiPlugins.build().disable::<SolveUiPlugin>())
        .run();
}
