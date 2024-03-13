// SPDX-FileCopyrightText: 2023-2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::prelude::*;

use crate::app::AppState;

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(OnEnter(AppState::SelectPuzzle), PuzzleSelectSet::OnEnter)
            .configure_sets(
                Update,
                (PuzzleSelectSet::UserInput, PuzzleSelectSet::Events)
                    .chain()
                    .run_if(in_state(AppState::SelectPuzzle)),
            )
            .configure_sets(OnExit(AppState::SelectPuzzle), PuzzleSelectSet::OnExit)
            .configure_sets(OnEnter(AppState::LoadPuzzle), PuzzleLoadSet::OnEnter)
            .configure_sets(
                Update,
                PuzzleLoadSet::Events.run_if(in_state(AppState::LoadPuzzle)),
            )
            .configure_sets(
                OnEnter(AppState::SolvePuzzle),
                (PuzzleSolveSet::OnEnter, PuzzleSolveSet::PostOnEnter).chain(),
            )
            .configure_sets(
                Update,
                (
                    PuzzleSolveSet::UserInput,
                    PuzzleSolveSet::Events,
                    PuzzleSolveSet::EntityUpdates,
                )
                    .chain()
                    .run_if(in_state(AppState::SolvePuzzle)),
            )
            .configure_sets(OnExit(AppState::SolvePuzzle), PuzzleSolveSet::OnExit);
    }
}

#[derive(SystemSet, Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum PuzzleSelectSet {
    OnEnter,
    UserInput,
    Events,
    OnExit,
}

#[derive(SystemSet, Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum PuzzleLoadSet {
    OnEnter,
    Events,
}

#[derive(SystemSet, Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum PuzzleSolveSet {
    OnEnter,
    PostOnEnter,
    UserInput,
    Events,
    EntityUpdates,
    OnExit,
}
