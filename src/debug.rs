// SPDX-FileCopyrightText: 2023-2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
    transform::TransformSystem,
};

use crate::{
    action::{CellEvent, PuzzleSolveEvent},
    app::AppState,
    layout::{
        position::EntityRealigned,
        size::{EntityResized, WindowResized},
    },
    puzzle::Position,
    solve::cell::Cell,
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LogPlugin {
            filter: "info,wgpu_core=warn,wgpu_hal=warn,hanjie=debug".into(),
            level: Level::DEBUG,
            ..default()
        })
        .configure_sets(Update, DebugSet::Events)
        .configure_sets(
            PostUpdate,
            DebugSet::GlobalTransform.after(TransformSystem::TransformPropagate),
        )
        .add_systems(
            Update,
            (
                snoop_event::<PuzzleSolveEvent>,
                snoop_event::<CellEvent>,
                snoop_event::<WindowResized>,
                snoop_event::<EntityResized>,
                snoop_event::<EntityRealigned>,
            )
                .chain()
                .in_set(DebugSet::Events),
        )
        .add_systems(
            PostUpdate,
            print_cell_location
                .in_set(DebugSet::GlobalTransform)
                .run_if(in_state(AppState::Solve).and_then(run_once())),
        );
    }
}

#[derive(SystemSet, Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum DebugSet {
    Events,
    GlobalTransform,
}

fn print_cell_location(query: Query<(&GlobalTransform, &Transform, &Position), With<Cell>>) {
    for (global_transform, transform, position) in query.iter() {
        let translation = transform.translation;
        let global_translation = global_transform.translation();
        debug!(
            "Cell({},{}) is located at Translation({},{}) and GlobalTranslation({},{})",
            position.column,
            position.row,
            translation.x,
            translation.y,
            global_translation.x,
            global_translation.y
        );
    }
}

// fn snoop_asset_load<T: Asset + std::fmt::Debug>(
//     mut events: EventReader<AssetEvent<T>>,
//     assets: Res<Assets<T>>,
// ) {
//     for event in events.read() {
//         if let AssetEvent::LoadedWithDependencies { id } = event {
//             if let Some(asset) = assets.get(*id) {
//                 debug!("Snoop Asset Loaded: {:#?}", asset)
//             }
//         }
//     }
// }

fn snoop_event<T: Event + std::fmt::Debug>(mut events: EventReader<T>) {
    for event in events.read() {
        debug!("Snoop Event: {:?}", event);
    }
}
