// SPDX-FileCopyrightText: Copyright (c) 2023 Jack Wolfard
// SPDX-License-Identifier: Apache-2.0

use bevy::{
    prelude::*,
    render::camera::RenderTarget,
    window::{PrimaryWindow, WindowRef},
};

use crate::game::{
    cell::{is_inside_cell, Cell, CellActionEvent, CellPlugin, Location},
    grid::GridPlugin,
};

mod cell;
mod grid;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CellPlugin, GridPlugin))
            .add_systems(PreStartup, setup)
            .add_systems(Update, (handle_click, handle_world_action))
            .add_event::<WorldActionEvent>();
    }
}

#[derive(Component)]
struct MainCamera;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Action {
    Toggle,
    CrossOut,
    Mark,
}

#[derive(Event)]
struct WorldActionEvent {
    position: Vec2,
    action: Action,
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn convert_camera_coords_to_world(
    camera: &Camera,
    camera_transform: &GlobalTransform,
    window: &Window,
) -> Option<Vec2> {
    window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
}

fn get_action_from_mouse_click(buttons: Res<Input<MouseButton>>) -> Option<Action> {
    if buttons.just_pressed(MouseButton::Left) {
        Some(Action::Toggle)
    } else if buttons.just_pressed(MouseButton::Middle) {
        Some(Action::Mark)
    } else if buttons.just_pressed(MouseButton::Right) {
        Some(Action::CrossOut)
    } else {
        None
    }
}

fn handle_click(
    buttons: Res<Input<MouseButton>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    window_q: Query<&Window>,
    primary_window_q: Query<&Window, With<PrimaryWindow>>,
    mut ev_worldaction: EventWriter<WorldActionEvent>,
) {
    if let Some(action) = get_action_from_mouse_click(buttons) {
        let (camera, camera_transform) = camera_q.single();
        let primary_window = primary_window_q.single();
        let window = match camera.target {
            RenderTarget::Window(wref) => match wref {
                WindowRef::Entity(id) => window_q.get(id).unwrap(),
                WindowRef::Primary => primary_window,
            },
            _ => primary_window,
        };
        if let Some(position) = convert_camera_coords_to_world(camera, camera_transform, window) {
            debug!(
                "handle_click: {action:?} click at ({},{})",
                position.x, position.y
            );
            ev_worldaction.send(WorldActionEvent { position, action })
        }
    }
}

fn handle_world_action(
    mut ev_worldaction: EventReader<WorldActionEvent>,
    mut ev_cellaction: EventWriter<CellActionEvent>,
    query: Query<(Entity, &Location, &GlobalTransform), With<Cell>>,
) {
    for ev in ev_worldaction.iter() {
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
