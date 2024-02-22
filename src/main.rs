// SPDX-FileCopyrightText: 2023-2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::{log::LogPlugin, prelude::*, window::WindowTheme};

use crate::{
    action::ActionPlugin, app::AppPlugin, camera::CameraPlugin, cell::CellPlugin,
    feature::FeaturePlugin, grid::GridPlugin, input::InputPlugin, inspect::InspectPlugin,
    schedule::SchedulePlugin, ui::UiPlugin,
};

mod action;
mod app;
mod camera;
mod cell;
mod feature;
mod grid;
mod input;
mod schedule;
mod ui;

#[cfg(feature = "inspect")]
mod inspect;

#[cfg(not(feature = "inspect"))]
mod inspect {
    use crate::feature::FeaturePlugin;

    pub struct InspectPlugin;
    impl FeaturePlugin for InspectPlugin {}
}

fn main() {
    let mut app = App::new();
    app.add_plugins(
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
    .add_plugins(CellPlugin)
    .add_plugins(GridPlugin)
    .add_plugins(InputPlugin)
    .add_plugins(SchedulePlugin)
    .add_plugins(UiPlugin);
    InspectPlugin::load(&mut app);
    app.run();
}
