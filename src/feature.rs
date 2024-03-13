// SPDX-FileCopyrightText: 2023-2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct FeaturePlugins;

impl PluginGroup for FeaturePlugins {
    #[allow(unused_mut, clippy::let_and_return)]
    fn build(self) -> bevy::app::PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        #[cfg(feature = "debug")]
        {
            group = group.add(crate::debug::DebugPlugin);
        }

        #[cfg(feature = "inspect")]
        {
            group = group.add(crate::inspect::InspectPlugin);
        }

        group
    }
}
