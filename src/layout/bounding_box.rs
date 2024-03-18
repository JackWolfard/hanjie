// SPDX-FileCopyrightText: 2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::{prelude::*, transform::TransformSystem};

pub struct BoundingBoxPlugin;

impl Plugin for BoundingBoxPlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<BoundingBoxGizmos>().add_systems(
            PostUpdate,
            wireframe.after(TransformSystem::TransformPropagate),
        );
    }
}

#[derive(Component)]
pub struct BoundingBox {
    primitive: Rectangle,
    color: Color,
}

impl BoundingBox {
    pub fn new(width: f32, height: f32, color: Color) -> Self {
        Self {
            primitive: Rectangle::new(width, height),
            color,
        }
    }

    pub fn size(&self) -> Vec2 {
        self.primitive.size()
    }

    pub fn resize(&mut self, size: Vec2) {
        self.primitive = Rectangle::from_size(size);
    }
}

#[derive(GizmoConfigGroup, Default, Reflect)]
struct BoundingBoxGizmos;

fn wireframe(
    mut gizmos: Gizmos<BoundingBoxGizmos>,
    query: Query<(&BoundingBox, &GlobalTransform)>,
) {
    for (bounding_box, transform) in query.iter() {
        let translation = transform.translation().xy();
        // let rotation = transform.rotation().to_euler(EulerRot::YXZ).2;
        gizmos.primitive_2d(bounding_box.primitive, translation, 0.0, bounding_box.color);
        gizmos.primitive_2d(Circle::new(5.0), translation, 0.0, bounding_box.color);
    }
}
