// SPDX-FileCopyrightText: 2023-2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use crate::feature::FeaturePlugin;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct InspectPlugin;

impl FeaturePlugin for InspectPlugin {
    fn load(app: &mut App) {
        app.add_plugins(InspectPlugin);
    }
}

impl Plugin for InspectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldInspectorPlugin::new());
    }
}
