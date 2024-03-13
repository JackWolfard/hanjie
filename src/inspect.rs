// SPDX-FileCopyrightText: 2023-2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::prelude::*;

pub struct InspectPlugin;

impl Plugin for InspectPlugin {
    #[allow(unused_variables)]
    fn build(&self, app: &mut App) {
        #[cfg(feature = "inspect")]
        {
            use bevy_inspector_egui::quick::WorldInspectorPlugin;
            app.add_plugins(WorldInspectorPlugin::new());
        }
    }
}
