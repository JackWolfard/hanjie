// SPDX-FileCopyrightText: 2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::{asset::LoadedFolder, prelude::*};

use crate::{
    action::{PuzzleSelectAction, PuzzleSelectEvent, PuzzleSelectState},
    app::AppState,
    puzzle::{Puzzle, PuzzlesFolder},
    schedule::SelectSet,
    ui::despawn_screen,
};

const PUZZLE_WIDTH: Val = Val::Vw(80.0);
const PUZZLE_HEIGHT: Val = Val::Vh(80.0);
const PUZZLE_GAP: Val = Val::Px(20.0);
const PUZZLE_SIZE: Val = Val::Px(100.0);
const PUZZLE_BORDER: Val = Val::Px(5.0);
const PUZZLE_BORDER_COLOR: Color = Color::WHITE;
const PUZZLE_BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
const PUZZLE_OUTLINE_COLOR: Color = Color::WHITE;
const PUZZLE_OUTLINE_SIZE: Val = Val::Px(10.0);

pub struct SelectUiPlugin;

impl Plugin for SelectUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::Select),
            spawn_puzzle_select_screen.in_set(SelectSet::OnEnter),
        )
        .add_systems(
            Update,
            (handle_ui_input, outline_hovered_button_system).in_set(SelectSet::UserInput),
        )
        .add_systems(
            OnExit(AppState::Select),
            despawn_screen::<SelectScreen>.in_set(SelectSet::OnExit),
        );
    }
}

#[derive(Component)]
struct SelectScreen;

#[derive(Component)]
struct SelectablePuzzle {
    name: Box<str>,
    index: usize,
}

fn spawn_puzzle_select_screen(
    mut commands: Commands,
    puzzles_folder: Res<PuzzlesFolder>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    puzzles: Res<Assets<Puzzle>>,
) {
    let screen = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    };

    let title = TextBundle::from_section(
        "Hanjie",
        TextStyle {
            font_size: 100.0,
            color: Color::WHITE,
            ..default()
        },
    )
    .with_text_justify(JustifyText::Center);

    let content = NodeBundle {
        style: Style {
            width: PUZZLE_WIDTH,
            height: PUZZLE_HEIGHT,
            flex_wrap: FlexWrap::Wrap,
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            row_gap: PUZZLE_GAP,
            column_gap: PUZZLE_GAP,
            ..default()
        },
        ..default()
    };

    let folder: &LoadedFolder = loaded_folders.get(&puzzles_folder.folder).unwrap();

    let selectable_puzzles: Vec<SelectablePuzzle> = folder
        .handles
        .iter()
        .enumerate()
        .filter_map(|(i, h)| {
            h.id().try_typed::<Puzzle>().ok().and_then(|p| {
                puzzles.get(p).map(|puzzle| SelectablePuzzle {
                    name: puzzle.name.clone(),
                    index: i,
                })
            })
        })
        .collect();

    let puzzles: Vec<(ButtonBundle, TextBundle, SelectablePuzzle)> = selectable_puzzles
        .into_iter()
        .map(|puzzle| {
            (
                ButtonBundle {
                    style: Style {
                        width: PUZZLE_SIZE,
                        height: PUZZLE_SIZE,
                        border: UiRect::all(PUZZLE_BORDER),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: PUZZLE_BORDER_COLOR.into(),
                    background_color: PUZZLE_BACKGROUND_COLOR.into(),
                    ..default()
                },
                TextBundle::from_section(
                    puzzle.name.clone(),
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                puzzle,
            )
        })
        .collect();

    let screen = commands.spawn((screen, SelectScreen)).id();
    let title = commands.spawn(title).id();
    let content = commands.spawn(content).id();
    let puzzles: Vec<Entity> = puzzles
        .into_iter()
        .map(|(button, label, puzzle)| {
            let button = commands.spawn(button).id();
            let label = commands.spawn(label).id();
            let puzzle = commands.spawn(puzzle).id();
            commands.entity(button).push_children(&[label, puzzle]);
            button
        })
        .collect();

    commands.entity(screen).push_children(&[title, content]);
    commands.entity(content).push_children(&puzzles);
}

#[allow(clippy::type_complexity)]
fn handle_ui_input(
    interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    selectable_puzzle_q: Query<&SelectablePuzzle>,
    mut ev_puzzle_select: EventWriter<PuzzleSelectEvent>,
) {
    for (interaction, children) in &interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            for child in children {
                if let Ok(SelectablePuzzle { index, .. }) = selectable_puzzle_q.get(*child) {
                    ev_puzzle_select.send(PuzzleSelectEvent {
                        action: PuzzleSelectAction::Select,
                        state: PuzzleSelectState::Puzzle(*index),
                    });
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn outline_hovered_button_system(
    mut commands: Commands,
    mut node_query: Query<
        (Entity, &Interaction, Option<&mut Outline>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (entity, interaction, maybe_outline) in node_query.iter_mut() {
        let outline_color = if matches!(interaction, Interaction::Hovered) {
            PUZZLE_OUTLINE_COLOR
        } else {
            Color::NONE
        };
        if let Some(mut outline) = maybe_outline {
            outline.color = outline_color;
        } else {
            commands.entity(entity).insert(Outline::new(
                PUZZLE_OUTLINE_SIZE,
                Val::ZERO,
                Color::NONE,
            ));
        }
    }
}
