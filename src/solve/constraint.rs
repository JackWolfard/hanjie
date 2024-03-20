// SPDX-FileCopyrightText: 2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::prelude::*;

use crate::{
    layout::{
        bounding_box::BoundingBox,
        position::{self, Alignable},
        size::{self, Resizable},
    },
    puzzle::{Constraint, GridSize},
};

pub struct ConstraintPlugin;

impl Plugin for ConstraintPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Bundle)]
struct ConstraintBundle {
    constraint: Constraint,
    #[bundle()]
    spatial_bundle: SpatialBundle,
    bounding_box: BoundingBox,
    alignable: Alignable,
    resizable: Resizable,
}

// root: cyan
// grid: green
// cell: pink
// constraint: yellow

impl ConstraintBundle {
    fn new(constraint: Constraint, size: &GridSize) -> Self {
        let (alignable, resizable) = match constraint {
            Constraint::Line(line, _) => {
                let alignable = Alignable::new(
                    position::Reference::Parent,
                    position::Alignment::Line(*size, line),
                );
                let resizable = match line {
                    crate::puzzle::Line::Column(_) => Resizable::new(
                        size::ResizableField {
                            constraint: size::Constraint::Fr(size.columns),
                            reference: size::Reference::Parent,
                        },
                        size::ResizableField {
                            constraint: size::Constraint::Fill,
                            reference: size::Reference::Intersect(
                                size::EntityReference::Grandparent,
                                size::EntityReference::Parent,
                            ),
                        },
                        None,
                    ),
                    crate::puzzle::Line::Row(_) => Resizable::new(
                        size::ResizableField {
                            constraint: size::Constraint::Fill,
                            reference: size::Reference::Intersect(
                                size::EntityReference::Grandparent,
                                size::EntityReference::Parent,
                            ),
                        },
                        size::ResizableField {
                            constraint: size::Constraint::Fr(size.rows),
                            reference: size::Reference::Parent,
                        },
                        None,
                    ),
                };
                (alignable, resizable)
            }
            Constraint::Position(_, _) => todo!("constraint for position"),
        };

        Self {
            constraint,
            spatial_bundle: SpatialBundle::default(),
            bounding_box: BoundingBox::init(Color::YELLOW),
            alignable,
            resizable,
        }
    }
}

pub(super) fn spawn(builder: &mut ChildBuilder, constraint: Constraint, size: &GridSize) {
    builder.spawn(ConstraintBundle::new(constraint, size));
}
