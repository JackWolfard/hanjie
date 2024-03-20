// SPDX-FileCopyrightText: 2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::{asset::LoadedFolder, prelude::*};
use bevy_common_assets::ron::RonAssetPlugin;
use serde::{Deserialize, Serialize};

use crate::{
    app::AppState,
    schedule::{LoadSet, SolveSet},
};

pub struct PuzzlePlugin;

impl Plugin for PuzzlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PuzzlesFolder>()
            .init_resource::<ActivePuzzle>()
            .add_plugins(RonAssetPlugin::<Puzzle>::new(&["ron"]))
            .add_systems(
                OnEnter(AppState::Load),
                load_puzzles.in_set(LoadSet::OnEnter),
            )
            .add_systems(Update, transition_on_load.in_set(LoadSet::Events))
            .add_systems(
                OnExit(AppState::Solve),
                unset_active_puzzle.in_set(SolveSet::OnExit),
            );
    }
}

fn load_puzzles(asset_server: Res<AssetServer>, mut puzzles: ResMut<PuzzlesFolder>) {
    puzzles.folder = asset_server.load_folder("puzzles");
}

fn unset_active_puzzle(mut active_puzzle: ResMut<ActivePuzzle>) {
    active_puzzle.puzzle = None;
}

fn transition_on_load(
    mut events: EventReader<AssetEvent<LoadedFolder>>,
    puzzles: Res<PuzzlesFolder>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for event in events.read() {
        if event.is_loaded_with_dependencies(&puzzles.folder) {
            next_state.set(AppState::Select);
        }
    }
}

#[derive(Resource, Default)]
pub struct PuzzlesFolder {
    pub folder: Handle<LoadedFolder>,
}

#[derive(Resource, Default)]
pub struct ActivePuzzle {
    pub puzzle: Option<Puzzle>,
}

#[derive(Deserialize, Serialize, Asset, TypePath, Debug, Clone)]
pub struct Puzzle {
    pub name: Box<str>,
    pub grid: GridSize,
    pub constraints: Box<[Constraint]>,
}

#[derive(Component, Deserialize, Serialize, Debug, Clone, Copy)]
pub struct GridSize {
    pub columns: i32,
    pub rows: i32,
}

impl From<GridSize> for Vec2 {
    fn from(grid: GridSize) -> Self {
        Vec2::new(grid.columns as f32, grid.rows as f32)
    }
}

#[derive(Component, Deserialize, Serialize, Debug, Clone)]
pub enum Constraint {
    Line(Line, Box<[Condition]>),
    Position(Position, Condition),
}

#[derive(Component, Deserialize, Serialize, Debug, Clone, Copy)]
pub enum Line {
    Column(i32),
    Row(i32),
}

#[derive(Component, Deserialize, Serialize, Debug, Copy, Clone)]
pub struct Position {
    pub column: i32,
    pub row: i32,
}

impl From<Position> for Vec2 {
    fn from(position: Position) -> Self {
        Vec2::new(position.column as f32, position.row as f32)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum Condition {
    Number(i32),
    Wildcard,
}
