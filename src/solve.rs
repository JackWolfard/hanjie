// SPDX-FileCopyrightText: 2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::prelude::*;

use crate::{
    app::AppState,
    puzzle::{ActivePuzzle, Puzzle},
    schedule::PuzzleSolveSet,
    solve::{cell::CellPlugin, grid::GridPlugin},
};

pub mod cell;
pub mod grid;

pub struct SolvePlugin;

impl Plugin for SolvePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CellPlugin)
            .add_plugins(GridPlugin)
            .add_systems(
                OnEnter(AppState::SolvePuzzle),
                spawn.in_set(PuzzleSolveSet::OnEnter),
            )
            .add_systems(
                OnExit(AppState::SolvePuzzle),
                despawn.in_set(PuzzleSolveSet::OnExit),
            );
    }
}

#[derive(Component)]
struct SolvePuzzleRoot;

#[derive(Bundle)]
pub struct SolvePuzzleBundle {
    #[bundle()]
    sprite_bundle: SpriteBundle,
}

impl SolvePuzzleBundle {
    pub fn new() -> SolvePuzzleBundle {
        SolvePuzzleBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::CYAN,
                    custom_size: Some(Vec2::splat(100.0)),
                    ..default()
                },
                ..default()
            },
        }
    }
}

fn spawn(mut commands: Commands, active_puzzle: Res<ActivePuzzle>, puzzles: Res<Assets<Puzzle>>) {
    let handle = active_puzzle.handle.as_ref().unwrap();
    let puzzle = puzzles.get(handle).unwrap();
    commands
        .spawn((SolvePuzzleBundle::new(), SolvePuzzleRoot))
        .with_children(|parent| grid::spawn(parent, puzzle));
}

fn despawn(mut commands: Commands, query: Query<Entity, With<SolvePuzzleRoot>>) {
    let root = query.single();
    commands.entity(root).despawn_recursive();
}
