// SPDX-FileCopyrightText: 2023-2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::{math::AspectRatio, prelude::*};

use crate::{
    action::{CellEvent, PuzzleSolveAction, PuzzleSolveState},
    layout::{
        bounding_box::BoundingBox,
        position::{self, Alignable},
        size::{self, EntityResized, Resizable},
    },
    puzzle::{GridSize, Position},
    schedule::SolveSet,
};

const CELL_CLEARED_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
const CELL_FILLED_COLOR: Color = Color::rgb(0.36, 0.58, 0.66);
const CELL_CROSSEDOUT_COLOR: Color = Color::rgb(0.66, 0.36, 0.36);
const CELL_MARKED_COLOR: Color = Color::rgb(0.54, 0.66, 0.36);

pub struct CellPlugin;

impl Plugin for CellPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CellSize>().add_systems(
            Update,
            (
                cell_resize.in_set(SolveSet::Events),
                handle_cell_action.in_set(SolveSet::EntityUpdates),
            ),
        );
    }
}

#[derive(Bundle)]
pub struct CellBundle {
    #[bundle()]
    sprite_bundle: SpriteBundle,
    bounding_box: BoundingBox,
    alignable: Alignable,
    resizable: Resizable,
    position: Position,
    cell: Cell,
}

#[derive(Component, Debug, Default)]
pub struct Cell {
    state: CellState,
}

#[derive(Resource, Debug, Default)]
pub struct CellSize {
    pub size: f32,
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
    pub fn new(row: i32, column: i32, size: &GridSize) -> Self {
        let cell: Cell = default();
        let color: Color = cell.state.color();
        CellBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite { color, ..default() },
                visibility: Visibility::Visible,
                ..default()
            },
            bounding_box: BoundingBox::init(Color::PINK),
            alignable: Alignable::new(
                position::Reference::Parent,
                position::Alignment::Grid(
                    *size,
                    Position { column, row },
                    Some(position::Spacing::Even),
                ),
            ),
            resizable: Resizable::new(
                size::ResizableField {
                    constraint: size::Constraint::Fr(size.columns + 1),
                    reference: size::Reference::Parent,
                },
                size::ResizableField {
                    constraint: size::Constraint::Fr(size.rows + 1),
                    reference: size::Reference::Parent,
                },
                Some(AspectRatio::new(1.0, 1.0)),
            ),
            position: Position { column, row },
            cell,
        }
    }
}

pub fn is_inside_cell(cell_size: &CellSize, cell_position: Vec3, position: Vec2) -> bool {
    let size = cell_size.size;
    let c = cell_position.xy() - Vec2::splat(size / 2.0);
    position.x >= c.x && position.y >= c.y && position.x <= c.x + size && position.y <= c.y + size
}

fn handle_cell_action(
    mut events: EventReader<CellEvent>,
    mut query: Query<(&mut Cell, &mut Sprite)>,
) {
    for CellEvent { action, state } in events.read() {
        if let PuzzleSolveState::Entity(entity) = state {
            if let Ok((mut cell, mut sprite)) = query.get_mut(*entity) {
                apply_action_to_cell(action, &mut cell);
                update_cell_sprite(&cell, &mut sprite);
            }
        }
    }
}

fn apply_action_to_cell(action: &PuzzleSolveAction, cell: &mut Cell) {
    cell.state = match action {
        PuzzleSolveAction::Toggle => match cell.state {
            CellState::Filled => CellState::Cleared,
            _ => CellState::Filled,
        },
        PuzzleSolveAction::Mark => CellState::Marked,
        PuzzleSolveAction::CrossOut => CellState::CrossedOut,
    }
}

fn update_cell_sprite(cell: &Cell, sprite: &mut Sprite) {
    sprite.color = cell.state.color();
}

fn cell_resize(
    mut events: EventReader<EntityResized>,
    mut cell_size: ResMut<CellSize>,
    mut cell_q: Query<&mut Sprite, With<Cell>>,
) {
    for EntityResized { entity, size } in events.read() {
        if let Ok(mut sprite) = cell_q.get_mut(*entity) {
            // hack: cell size changes many times over since many cells resize
            cell_size.size = size.min_element();
            sprite.custom_size = Some(*size);
        }
    }
}
