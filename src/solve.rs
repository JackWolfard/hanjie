// SPDX-FileCopyrightText: 2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::prelude::*;

use crate::{
    app::AppState,
    camera::MainCamera,
    layout::{
        bounding_box::BoundingBox,
        size::{self, Resizable},
    },
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
struct SolvePuzzleBundle {
    #[bundle()]
    spatial_bundle: SpatialBundle,
    bounding_box: BoundingBox,
    resizable: Resizable,
}

impl SolvePuzzleBundle {
    fn new(width: f32, height: f32) -> Self {
        Self {
            spatial_bundle: SpatialBundle::default(),
            bounding_box: BoundingBox::new(width, height, Color::CYAN),
            resizable: Resizable::new(
                size::Reference::Window,
                size::Constraint::Pct(0.6),
                size::Constraint::Pct(1.0),
                None,
            ),
        }
    }
}

impl Default for SolvePuzzleBundle {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

fn spawn(
    mut commands: Commands,
    camera_q: Query<&Camera, With<MainCamera>>,
    active_puzzle: Res<ActivePuzzle>,
    puzzles: Res<Assets<Puzzle>>,
) {
    let handle = active_puzzle.handle.as_ref().unwrap();
    let puzzle = puzzles.get(handle).unwrap();
    let camera = camera_q.single();
    let view = camera.logical_viewport_size().unwrap();
    let mut bundle = SolvePuzzleBundle::default();
    let size = bundle.resizable.constraints.apply(view);
    bundle.bounding_box.resize(size);
    commands
        .spawn((bundle, SolvePuzzleRoot))
        .with_children(|parent| grid::spawn(parent, puzzle));
}

fn despawn(mut commands: Commands, query: Query<Entity, With<SolvePuzzleRoot>>) {
    let root = query.single();
    commands.entity(root).despawn_recursive();
}
