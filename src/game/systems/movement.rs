use bevy::prelude::*;

use crate::{
    config::constants::{CANVAS_SIZE, DEFAULT_PADDLE_SIZE, PADDLE_SPEED},
    game::components::paddle::Paddle,
};

pub(in crate::game) fn paddle_controls(
    input: Res<ButtonInput<KeyCode>>,
    mut paddles: Query<&mut Transform, With<Paddle>>,
    time: Res<Time>,
) {
    for mut transform in &mut paddles {
        let direction = if input.pressed(KeyCode::ArrowLeft) {
            -1.0
        } else if input.pressed(KeyCode::ArrowRight) {
            1.0
        } else {
            0.0
        };

        transform.translation.x += direction * PADDLE_SPEED * time.delta_secs();

        let max_x = CANVAS_SIZE.x / 2.0 - DEFAULT_PADDLE_SIZE.x / 2.0;
        transform.translation.x = transform.translation.x.clamp(-max_x, max_x);
    }
}
