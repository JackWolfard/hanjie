// SPDX-FileCopyrightText: 2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use serde::{Deserialize, Serialize};

use crate::{
    app::AppState,
    schedule::{PuzzleLoadSet, PuzzleSolveSet},
};

pub struct PuzzlePlugin;

impl Plugin for PuzzlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActivePuzzle>()
            .add_plugins(RonAssetPlugin::<Puzzle>::new(&["ron"]))
            .add_systems(
                OnEnter(AppState::LoadPuzzle),
                load_puzzle.in_set(PuzzleLoadSet::OnEnter),
            )
            .add_systems(
                Update,
                transition_to_solve_if_active_puzzle_is_loaded.in_set(PuzzleLoadSet::Events),
            )
            .add_systems(
                OnExit(AppState::SolvePuzzle),
                unload_puzzle.in_set(PuzzleSolveSet::OnExit),
            );
    }
}

fn load_puzzle(asset_server: Res<AssetServer>, mut active_puzzle: ResMut<ActivePuzzle>) {
    active_puzzle.handle = Some(asset_server.load("puzzles/top-hat.ron"));
}

fn unload_puzzle(mut puzzles: ResMut<Assets<Puzzle>>, mut active_puzzle: ResMut<ActivePuzzle>) {
    if let Some(ref handle) = active_puzzle.handle {
        puzzles.remove(handle);
        active_puzzle.handle = None;
    }
}

fn transition_to_solve_if_active_puzzle_is_loaded(
    mut events: EventReader<AssetEvent<Puzzle>>,
    active_puzzle: Res<ActivePuzzle>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if let Some(ref handle) = active_puzzle.handle {
        for event in events.read() {
            if let AssetEvent::LoadedWithDependencies { id } = event {
                if handle.id() == *id {
                    next_state.set(AppState::SolvePuzzle)
                }
            }
        }
    }
}

#[derive(Resource, Default)]
pub struct ActivePuzzle {
    pub handle: Option<Handle<Puzzle>>,
}

#[derive(Deserialize, Serialize, Asset, TypePath, Debug)]
pub struct Puzzle {
    pub name: Box<str>,
    pub grid: GridSize,
    pub constraints: Box<[Constraint]>,
}

#[derive(Component, Deserialize, Serialize, Debug, Copy, Clone)]
pub struct GridSize {
    pub columns: i32,
    pub rows: i32,
}

impl From<GridSize> for Vec2 {
    fn from(grid: GridSize) -> Self {
        Vec2::new(grid.columns as f32, grid.rows as f32)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Constraint {
    Line(Location, Box<[Condition]>),
}

#[derive(Component, Deserialize, Serialize, Debug)]
pub enum Location {
    Row(i32),
    Column(i32),
    Position(LocationPosition),
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub struct LocationPosition {
    pub column: i32,
    pub row: i32,
}

impl From<LocationPosition> for Vec2 {
    fn from(location: LocationPosition) -> Self {
        Vec2::new(location.column as f32, location.row as f32)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Condition {
    Number(i32),
    Wildcard,
}
