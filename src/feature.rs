// SPDX-FileCopyrightText: 2023-2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::prelude::*;

pub trait FeaturePlugin {
    #[allow(unused_mut, unused_variables)]
    fn load(app: &mut App) {}
}
