// SPDX-FileCopyrightText: 2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::prelude::*;

use crate::{
    app::AppState,
    // puzzle::{ActivePuzzle, Puzzle},
    schedule::PuzzleSolveSet,
    ui::despawn_screen,
};

const PUZZLE_WIDTH_PCT: f32 = 60.0;
const PUZZLE_WIDTH: Val = Val::Vw(PUZZLE_WIDTH_PCT);
const PUZZLE_HEIGHT: Val = Val::Vh(100.0);
const PUZZLE_LEFT: Val = Val::Vw((100.0 - PUZZLE_WIDTH_PCT) / 2.0);
const PUZZLE_BACKGROUND_COLOR: Color = Color::CYAN;

const GRID_WIDTH: Val = Val::Percent(66.7);
const GRID_BACKGROUND_COLOR: Color = Color::GRAY;

pub struct SolveUiPlugin;

impl Plugin for SolveUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::SolvePuzzle),
            spawn_solve_puzzle_screen.in_set(PuzzleSolveSet::OnEnter),
        )
        .add_systems(
            OnExit(AppState::SolvePuzzle),
            despawn_screen::<SolvePuzzleScreen>.in_set(PuzzleSolveSet::OnExit),
        );
    }
}

#[derive(Component)]
struct SolvePuzzleScreen;

fn spawn_solve_puzzle_screen(
    mut commands: Commands,
    // puzzles: Res<Assets<Puzzle>>,
    // active_puzzle: Res<ActivePuzzle>,
) {
    let screen = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            // justify_content: JustifyContent::Start,
            // align_items: AlignItems::Center,
            // flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    };

    let mut title = TextBundle::from_section(
        "Hanjie",
        TextStyle {
            font_size: 100.0,
            color: Color::WHITE,
            ..default()
        },
    )
    .with_style(Style {
        align_self: AlignSelf::Start,
        ..default()
    })
    .with_text_justify(JustifyText::Center);
    title.z_index = ZIndex::Local(2);
    let title = title;

    let content = NodeBundle {
        style: Style {
            display: Display::Grid,
            position_type: PositionType::Absolute,
            width: PUZZLE_WIDTH,
            height: PUZZLE_HEIGHT,
            left: PUZZLE_LEFT,
            ..default()
        },
        background_color: PUZZLE_BACKGROUND_COLOR.into(),
        z_index: ZIndex::Local(0),
        ..default()
    };

    let debug_grid = NodeBundle {
        style: Style {
            width: GRID_WIDTH,
            aspect_ratio: Some(1.0),
            align_self: AlignSelf::End,
            justify_self: JustifySelf::End,
            ..default()
        },
        background_color: GRID_BACKGROUND_COLOR.into(),
        z_index: ZIndex::Local(1),
        ..default()
    };

    let screen = commands.spawn((screen, SolvePuzzleScreen)).id();
    let title = commands.spawn(title).id();
    let content = commands.spawn(content).id();
    let debug_grid = commands.spawn(debug_grid).id();
    // let handle = active_puzzle.handle.as_ref().unwrap();
    // let puzzle = puzzles.get(handle.id()).unwrap();
    // let grid = spawn_grid(&mut commands, &puzzle);

    commands.entity(screen).push_children(&[title, content]);
    commands.entity(content).add_child(debug_grid);
    // commands.entity(debug_grid).add_child(grid);
}
