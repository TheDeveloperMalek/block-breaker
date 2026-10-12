use bevy::prelude::*;

use crate::{
    app::state::AppState,
    config::constants::BALL_SIZE,
    game::{
        components::{ball::Ball, brick::Brick, wall::RespawnBallArea},
        systems::{collision::circle_aabb_normal, setup::spawn_bricks},
    },
};

pub(in crate::game) fn on_intersect_respawn_area(
    respawn_area: Single<(&Transform, &Sprite), With<RespawnBallArea>>,
    balls: Query<&Transform, With<Ball>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let (area_transform, sprite) = *respawn_area;

    let Some(size) = sprite.custom_size else {
        return;
    };

    let area_center = area_transform.translation.truncate();
    let area_half_size = size / 2.0;

    for ball in &balls {
        let position = ball.translation.truncate();

        if circle_aabb_normal(position, BALL_SIZE, area_center, area_half_size).is_some() {
            next_state.set(AppState::GameOver);
            return;
        }
    }
}

pub(in crate::game) fn spawn_bricks_when_cleared(
    bricks: Query<(), With<Brick>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    if bricks.is_empty() {
        spawn_bricks(&mut commands, &mut meshes, &mut materials, &asset_server);
    }
}
