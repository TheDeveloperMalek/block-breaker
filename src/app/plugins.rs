use bevy::{color::palettes::tailwind::SKY_950, prelude::*};

use crate::{
    app::state::AppState, audio::plugin::SoundEffectsPlugin, game::plugin::GamePlugin,
    ui::plugin::UiPlugin,
};

pub(crate) struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::from(SKY_950)))
            .init_state::<AppState>()
            .add_plugins((SoundEffectsPlugin, GamePlugin, UiPlugin));
    }
}
