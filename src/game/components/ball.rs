use bevy::prelude::{Component, Vec2};

#[derive(Component)]
pub(in crate::game) struct Ball;

#[derive(Component)]
pub(in crate::game) struct Velocity(pub(in crate::game) Vec2);
