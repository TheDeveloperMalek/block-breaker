use bevy::prelude::{Component, Plane2d, Vec2};

#[derive(Component)]
pub(in crate::game) struct Wall(pub(in crate::game) Plane2d);

#[derive(Component)]
pub(in crate::game) struct HalfSize(pub(in crate::game) Vec2);

#[derive(Component)]
pub(in crate::game) struct RespawnBallArea;
