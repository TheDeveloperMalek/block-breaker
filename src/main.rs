mod app;
mod audio;
mod config;
mod game;
mod ui;

use bevy::prelude::*;

use crate::app::plugins::AppPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AppPlugin)
        .run();
}
