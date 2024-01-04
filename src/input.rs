// SPDX-FileCopyrightText: 2023-2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::{
    prelude::*,
    render::camera::RenderTarget,
    window::{PrimaryWindow, WindowRef},
};

use crate::{
    action::{Action, WorldActionEvent},
    camera::MainCamera,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_click);
    }
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

fn map_click_to_action(
    buttons: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
) -> Option<Action> {
    let shift = keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    if buttons.just_pressed(MouseButton::Left) {
        if shift {
            Some(Action::Mark)
        } else {
            Some(Action::Toggle)
        }
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
    keys: Res<Input<KeyCode>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    window_q: Query<&Window>,
    primary_window_q: Query<&Window, With<PrimaryWindow>>,
    mut ev_worldaction: EventWriter<WorldActionEvent>,
) {
    if let Some(action) = map_click_to_action(buttons, keys) {
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
