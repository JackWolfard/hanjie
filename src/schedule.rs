// SPDX-FileCopyrightText: 2023-2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::prelude::*;

use crate::app::AppState;

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(OnEnter(AppState::PuzzleSelect), PuzzleSelectSet::OnEnter)
            .configure_sets(
                Update,
                PuzzleSelectSet::UserInput.run_if(in_state(AppState::PuzzleSelect)),
            )
            .configure_sets(OnExit(AppState::PuzzleSelect), PuzzleSelectSet::OnExit)
            .configure_sets(
                OnEnter(AppState::InGame),
                (InGameSet::OnEnter, InGameSet::PostOnEnter).chain(),
            )
            .configure_sets(
                Update,
                (
                    InGameSet::UserInput,
                    InGameSet::Events,
                    InGameSet::EntityUpdates,
                )
                    .chain()
                    .run_if(in_state(AppState::InGame)),
            )
            .configure_sets(OnExit(AppState::InGame), InGameSet::OnExit);
    }
}

#[derive(SystemSet, Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum PuzzleSelectSet {
    OnEnter,
    UserInput,
    OnExit,
}

#[derive(SystemSet, Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum InGameSet {
    OnEnter,
    PostOnEnter,
    UserInput,
    Events,
    EntityUpdates,
    OnExit,
}
