// SPDX-FileCopyrightText: 2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::{app::PluginGroupBuilder, prelude::*};

use crate::layout::{
    bounding_box::BoundingBoxPlugin, position::PositionPlugin, schedule::SchedulePlugin,
    size::SizePlugin,
};

pub mod bounding_box;
pub mod position;
mod schedule;
pub mod size;

pub struct LayoutPlugins;

impl PluginGroup for LayoutPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(BoundingBoxPlugin)
            .add(PositionPlugin)
            .add(SchedulePlugin)
            .add(SizePlugin)
    }
}
