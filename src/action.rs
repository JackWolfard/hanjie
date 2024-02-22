// SPDX-FileCopyrightText: 2023-2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::prelude::*;

use crate::{
    cell::{is_inside_cell, Cell, Location},
    schedule::InGameSet,
};

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameActionEvent>()
            .add_event::<CellActionEvent>()
            .add_systems(Update, handle_world_action.in_set(InGameSet::Events));
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Action {
    Toggle,
    CrossOut,
    Mark,
}

#[derive(Event)]
pub struct GameActionEvent {
    pub position: Vec2,
    pub action: Action,
}

#[derive(Event)]
pub struct CellActionEvent {
    pub entity: Entity,
    pub action: Action,
}

fn handle_world_action(
    mut ev_worldaction: EventReader<GameActionEvent>,
    mut ev_cellaction: EventWriter<CellActionEvent>,
    query: Query<(Entity, &Location, &GlobalTransform), With<Cell>>,
) {
    for ev in ev_worldaction.read() {
        for (entity, location, transform) in query.iter() {
            if is_inside_cell(transform.translation(), ev.position) {
                debug!(
                    "{:?} Cell({},{}) ",
                    ev.action, location.column, location.row
                );
                ev_cellaction.send(CellActionEvent {
                    entity,
                    action: ev.action,
                });
            }
        }
    }
}
