// SPDX-FileCopyrightText: 2023-2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::{
    prelude::*,
    render::camera::RenderTarget,
    window::{PrimaryWindow, WindowRef},
};

use crate::{
    action::{Action, GameActionEvent},
    app::AppState,
    camera::MainCamera,
    schedule::{InGameSet, PuzzleSelectSet},
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_game_key_press, handle_game_click)
                .chain()
                .in_set(InGameSet::UserInput),
        )
        .add_systems(
            Update,
            bevy::window::close_on_esc.in_set(PuzzleSelectSet::UserInput),
        );
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
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
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

fn handle_game_key_press(
    mut next_state: ResMut<NextState<AppState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::PuzzleSelect);
    }
}

fn handle_game_click(
    buttons: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    window_q: Query<&Window>,
    primary_window_q: Query<&Window, With<PrimaryWindow>>,
    mut ev_worldaction: EventWriter<GameActionEvent>,
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
            ev_worldaction.send(GameActionEvent { position, action });
        }
    }
}
