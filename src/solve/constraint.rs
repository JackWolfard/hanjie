// SPDX-FileCopyrightText: 2024 Jack Wolfard
//
// SPDX-License-Identifier: Apache-2.0

use bevy::{math::AspectRatio, prelude::*};

use crate::{
    layout::{
        bounding_box::BoundingBox,
        position::{self, Alignable},
        size::{self, Resizable},
    },
    puzzle::{Condition, Constraint, GridSize, Line},
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

impl ConstraintBundle {
    fn new(constraint: Constraint, size: &GridSize) -> Self {
        let (alignable, resizable) = match constraint {
            Constraint::Line(line, _) => {
                let alignable = Alignable::new(
                    position::Reference::Parent,
                    position::Alignment::Line(*size, line),
                );
                let resizable = match line {
                    Line::Column(_) => Resizable::new(
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
                    Line::Row(_) => Resizable::new(
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

#[derive(Bundle)]
struct ConditionBundle {
    condition: Condition,
    #[bundle()]
    spatial_bundle: SpatialBundle,
    bounding_box: BoundingBox,
    alignable: Alignable,
    resizable: Resizable,
}

impl ConditionBundle {
    fn new(condition: Condition, line: Line) -> Self {
        let alignment = match line {
            Line::Column(_) => position::Alignment::Standard(
                position::VerticalAlign::Center,
                position::HorizontalAlign::Center,
            ),
            Line::Row(_) => position::Alignment::Standard(
                position::VerticalAlign::Center,
                position::HorizontalAlign::Center,
            ),
        };
        let alignable = Alignable::new(position::Reference::Parent, alignment);

        let resizable = Resizable::new(
            size::ResizableField {
                constraint: size::Constraint::Fill,
                reference: size::Reference::Parent,
            },
            size::ResizableField {
                constraint: size::Constraint::Fill,
                reference: size::Reference::Parent,
            },
            Some(AspectRatio::new(1.0, 1.0)),
        );

        Self {
            condition,
            spatial_bundle: SpatialBundle::default(),
            bounding_box: BoundingBox::init(Color::ORANGE),
            alignable,
            resizable,
        }
    }
}

pub(super) fn spawn(builder: &mut ChildBuilder, constraint: Constraint, size: &GridSize) {
    builder
        .spawn(ConstraintBundle::new(constraint.clone(), size))
        .with_children(|parent| {
            if let Constraint::Line(line, conditions) = constraint {
                for condition in conditions.iter() {
                    parent.spawn(ConditionBundle::new(*condition, line));
                }
            }
        });
}
