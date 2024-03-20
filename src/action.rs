// SPDX-FileCopyrightText: 2023-2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::{asset::LoadedFolder, prelude::*};

use crate::{
    app::AppState,
    puzzle::{ActivePuzzle, Puzzle, PuzzlesFolder},
    schedule::{SelectSet, SolveSet},
    solve::cell::{is_inside_cell, Cell, CellSize},
};

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PuzzleSelectEvent>()
            .add_event::<PuzzleSolveEvent>()
            .add_event::<CellEvent>()
            .add_systems(Update, handle_puzzle_select.in_set(SelectSet::Events))
            .add_systems(Update, handle_puzzle_solve.in_set(SolveSet::Events));
    }
}

#[derive(Event, Debug)]
pub struct PuzzleSelectEvent {
    pub action: PuzzleSelectAction,
    pub state: PuzzleSelectState,
}

#[derive(Debug, Clone, Copy)]
pub enum PuzzleSelectAction {
    Select,
}

#[derive(Debug, Clone, Copy)]
pub enum PuzzleSelectState {
    Puzzle(usize),
}

#[derive(Event, Debug)]
pub struct PuzzleSolveEvent {
    pub action: PuzzleSolveAction,
    pub state: PuzzleSolveState,
}

#[derive(Debug, Clone, Copy)]
pub enum PuzzleSolveAction {
    CrossOut,
    Mark,
    Toggle,
}

#[derive(Debug, Clone, Copy)]
pub enum PuzzleSolveState {
    WorldPosition(Vec2),
    Entity(Entity),
}

#[derive(Event, Debug)]
pub struct CellEvent {
    pub action: PuzzleSolveAction,
    pub state: PuzzleSolveState,
}

fn handle_puzzle_select(
    mut events: EventReader<PuzzleSelectEvent>,
    puzzles: Res<Assets<Puzzle>>,
    puzzles_folder: Res<PuzzlesFolder>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    mut active_puzzle: ResMut<ActivePuzzle>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let folder: Option<&LoadedFolder> = loaded_folders.get(&puzzles_folder.folder);
    if let Some(folder) = folder {
        for event in events.read() {
            match event {
                PuzzleSelectEvent {
                    action: PuzzleSelectAction::Select,
                    state: PuzzleSelectState::Puzzle(index),
                } => {
                    let handle = folder.handles.get(*index).unwrap().clone();
                    if let Ok(id) = handle.id().try_typed::<Puzzle>() {
                        active_puzzle.puzzle = puzzles.get(id).cloned();
                        next_state.set(AppState::Solve);
                    }
                }
            }
        }
    }
}

fn handle_puzzle_solve(
    mut puzzle_solve_events: EventReader<PuzzleSolveEvent>,
    query: Query<(Entity, &GlobalTransform), With<Cell>>,
    cell_size: Res<CellSize>,
    mut cell_events: EventWriter<CellEvent>,
) {
    for PuzzleSolveEvent { action, state } in puzzle_solve_events.read() {
        if let PuzzleSolveState::WorldPosition(position) = state {
            for (entity, transform) in query.iter() {
                if is_inside_cell(&cell_size, transform.translation(), *position) {
                    cell_events.send(CellEvent {
                        action: *action,
                        state: PuzzleSolveState::Entity(entity),
                    });
                }
            }
        }
    }
}
