use bevy::prelude::*;

use crate::{
    app::state::AppState,
    game::{
        resources::Score,
        systems::{
            collision::ball_movement,
            lifecycle::{on_intersect_respawn_area, spawn_bricks_when_cleared},
            movement::paddle_controls,
            setup::{new_game, setup},
        },
    },
};

pub(crate) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(Startup, setup)
            .add_systems(OnEnter(AppState::Playing), new_game)
            .add_systems(
                FixedUpdate,
                (
                    paddle_controls,
                    ball_movement,
                    spawn_bricks_when_cleared,
                    on_intersect_respawn_area,
                )
                    .chain()
                    .run_if(in_state(AppState::Playing)),
            );
    }
}
