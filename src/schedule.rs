// SPDX-FileCopyrightText: 2023-2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::prelude::*;

use crate::app::AppState;

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(OnEnter(AppState::Select), SelectSet::OnEnter)
            .configure_sets(
                Update,
                (SelectSet::UserInput, SelectSet::Events)
                    .chain()
                    .run_if(in_state(AppState::Select)),
            )
            .configure_sets(OnExit(AppState::Select), SelectSet::OnExit)
            .configure_sets(OnEnter(AppState::Load), LoadSet::OnEnter)
            .configure_sets(Update, LoadSet::Events.run_if(in_state(AppState::Load)))
            .configure_sets(
                OnEnter(AppState::Solve),
                (SolveSet::OnEnter, SolveSet::PostOnEnter).chain(),
            )
            .configure_sets(
                Update,
                (
                    SolveSet::UserInput,
                    SolveSet::Events,
                    SolveSet::EntityUpdates,
                )
                    .chain()
                    .run_if(in_state(AppState::Solve)),
            )
            .configure_sets(OnExit(AppState::Solve), SolveSet::OnExit);
    }
}

#[derive(SystemSet, Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum SelectSet {
    OnEnter,
    UserInput,
    Events,
    OnExit,
}

#[derive(SystemSet, Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum LoadSet {
    OnEnter,
    Events,
}

#[derive(SystemSet, Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum SolveSet {
    OnEnter,
    PostOnEnter,
    UserInput,
    Events,
    EntityUpdates,
    OnExit,
}
