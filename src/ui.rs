// SPDX-FileCopyrightText: 2023-2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::prelude::*;

use crate::{app::AppState, schedule::PuzzleSelectSet};

const PUZZLES: i32 = 15;
const PUZZLE_WIDTH: Val = Val::Vw(80.0);
const PUZZLE_HEIGHT: Val = Val::Vh(80.0);
const PUZZLE_GAP: Val = Val::Px(20.0);
const PUZZLE_SIZE: Val = Val::Px(100.0);
const PUZZLE_BORDER: Val = Val::Px(5.0);
const PUZZLE_BORDER_COLOR: Color = Color::WHITE;
const PUZZLE_BACKGROUND_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
const PUZZLE_OUTLINE_COLOR: Color = Color::WHITE;
const PUZZLE_OUTLINE_SIZE: Val = Val::Px(10.0);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::PuzzleSelect),
            spawn_puzzle_select_screen.in_set(PuzzleSelectSet::OnEnter),
        )
        .add_systems(
            Update,
            (handle_ui_input, outline_hovered_button_system).in_set(PuzzleSelectSet::UserInput),
        )
        .add_systems(
            OnExit(AppState::PuzzleSelect),
            despawn_screen::<PuzzleSelectScreen>.in_set(PuzzleSelectSet::OnExit),
        );
    }
}

#[derive(Component)]
struct PuzzleSelectScreen;

fn spawn_puzzle_select_screen(mut commands: Commands) {
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
    .with_text_alignment(TextAlignment::Center);

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

    let puzzles: Vec<(ButtonBundle, TextBundle)> = (0..PUZZLES)
        .map(|_| {
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
                    "Button",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
            )
        })
        .collect();

    let screen = commands.spawn((screen, PuzzleSelectScreen)).id();
    let title = commands.spawn(title).id();
    let content = commands.spawn(content).id();
    let puzzles: Vec<Entity> = puzzles
        .into_iter()
        .map(|(puzzle, label)| {
            let puzzle = commands.spawn(puzzle).id();
            let label = commands.spawn(label).id();
            commands.entity(puzzle).push_children(&[label]);
            puzzle
        })
        .collect();

    commands.entity(screen).push_children(&[title, content]);
    commands.entity(content).push_children(&puzzles);
}

// From Bevy game menu example
// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

#[allow(clippy::type_complexity)]
fn handle_ui_input(
    interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, children) in &interaction_query {
        if let Ok(mut text) = text_query.get_mut(children[0]) {
            match interaction {
                Interaction::Pressed => text.sections[0].value = "Press".to_string(),
                Interaction::Hovered => text.sections[0].value = "Hover".to_string(),
                Interaction::None => text.sections[0].value = "Button".to_string(),
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
