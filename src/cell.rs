// SPDX-FileCopyrightText: 2023-2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::prelude::*;

use crate::{
    action::{Action, CellActionEvent},
    schedule::InGameSet,
};

const CELL_CLEARED_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
const CELL_FILLED_COLOR: Color = Color::rgb(0.36, 0.58, 0.66);
const CELL_CROSSEDOUT_COLOR: Color = Color::rgb(0.66, 0.36, 0.36);
const CELL_MARKED_COLOR: Color = Color::rgb(0.54, 0.66, 0.36);
const CELL_SIZE: f32 = 50.0;
const CELL_GUTTER: f32 = 10.0;

pub struct CellPlugin;

impl Plugin for CellPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_cell_action.in_set(InGameSet::EntityUpdates));
    }
}

#[derive(Component)]
pub struct Location {
    pub column: i32,
    pub row: i32,
}

#[derive(Bundle)]
pub struct CellBundle {
    location: Location,
    cell: Cell,
    #[bundle()]
    sprite_bundle: SpriteBundle,
}

#[derive(Component, Default)]
pub struct Cell {
    state: CellState,
}

#[derive(Debug, Default, Clone, PartialEq)]
enum CellState {
    #[default]
    Cleared,
    Filled,
    CrossedOut,
    Marked,
}

impl CellState {
    fn color(&self) -> Color {
        match self {
            CellState::Cleared => CELL_CLEARED_COLOR,
            CellState::Filled => CELL_FILLED_COLOR,
            CellState::CrossedOut => CELL_CROSSEDOUT_COLOR,
            CellState::Marked => CELL_MARKED_COLOR,
        }
    }
}

impl CellBundle {
    pub fn new(column: i32, row: i32) -> CellBundle {
        let cell: Cell = default();
        let color: Color = cell.state.color();
        CellBundle {
            location: Location { column, row },
            cell,
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::splat(CELL_SIZE)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::from((
                    Vec2::splat(CELL_SIZE + CELL_GUTTER) * Vec2::new(column as f32, row as f32),
                    0.0,
                ))),
                ..default()
            },
        }
    }
}

pub fn is_inside_cell(cell_position: Vec3, position: Vec2) -> bool {
    let c = Vec2::new(
        cell_position.x - CELL_SIZE / 2.0,
        cell_position.y - CELL_SIZE / 2.0,
    );
    position.x >= c.x
        && position.y >= c.y
        && position.x <= c.x + CELL_SIZE
        && position.y <= c.y + CELL_SIZE
}

fn handle_cell_action(
    mut ev_cellaction: EventReader<CellActionEvent>,
    mut query: Query<(&mut Cell, &mut Sprite)>,
) {
    for ev in ev_cellaction.read() {
        if let Ok((mut cell, mut sprite)) = query.get_mut(ev.entity) {
            apply_action_to_cell(ev.action, &mut cell);
            update_cell_sprite(&cell, &mut sprite);
        }
    }
}

fn apply_action_to_cell(action: Action, cell: &mut Cell) {
    cell.state = match action {
        Action::Toggle => match cell.state {
            CellState::Filled => CellState::Cleared,
            _ => CellState::Filled,
        },
        Action::Mark => CellState::Marked,
        Action::CrossOut => CellState::CrossedOut,
    }
}

fn update_cell_sprite(cell: &Cell, sprite: &mut Sprite) {
    sprite.color = cell.state.color();
}
