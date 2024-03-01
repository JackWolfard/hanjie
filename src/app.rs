// SPDX-FileCopyrightText: 2023-2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::prelude::*;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>();
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum AppState {
    #[default]
    PuzzleSelect,
    InGame,
}
