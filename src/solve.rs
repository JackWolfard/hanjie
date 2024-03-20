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
    puzzle::ActivePuzzle,
    schedule::SolveSet,
    solve::{cell::CellPlugin, constraint::ConstraintPlugin, grid::GridPlugin},
};

pub mod cell;
pub mod constraint;
pub mod grid;

pub struct SolvePlugin;

impl Plugin for SolvePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CellPlugin)
            .add_plugins(ConstraintPlugin)
            .add_plugins(GridPlugin)
            .add_systems(
                OnEnter(AppState::Solve),
                (spawn, flow_initial_layout)
                    .chain()
                    .in_set(SolveSet::OnEnter),
            )
            .add_systems(OnExit(AppState::Solve), despawn.in_set(SolveSet::OnExit));
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
    fn new() -> Self {
        Self {
            spatial_bundle: SpatialBundle::default(),
            bounding_box: BoundingBox::init(Color::CYAN),
            resizable: Resizable::new(
                size::ResizableField {
                    constraint: size::Constraint::Pct(0.6),
                    reference: size::Reference::Window,
                },
                size::ResizableField {
                    constraint: size::Constraint::Fill,
                    reference: size::Reference::Window,
                },
                None,
            ),
        }
    }
}

fn spawn(mut commands: Commands, active_puzzle: Res<ActivePuzzle>) {
    let puzzle = active_puzzle.puzzle.as_ref().unwrap();
    commands
        .spawn((SolvePuzzleBundle::new(), SolvePuzzleRoot))
        .with_children(|parent| grid::spawn(parent, puzzle));
}

fn flow_initial_layout(
    _camera_q: Query<&Camera, With<MainCamera>>,
    mut _layout_ev: EventWriter<size::WindowResized>,
) {
    // let camera = camera_q.single();
    // let view = camera.logical_viewport_size().unwrap();
    // trick to initialize size w/o explicit sizes
    // layout_ev.send(size::WindowResized { size: view });
}

fn despawn(mut commands: Commands, query: Query<Entity, With<SolvePuzzleRoot>>) {
    let root = query.single();
    commands.entity(root).despawn_recursive();
}
