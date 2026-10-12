use bevy::prelude::States;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub(crate) enum AppState {
    Playing,
    #[default]
    GameOver,
}
