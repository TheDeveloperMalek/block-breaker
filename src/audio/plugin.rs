use bevy::{
    audio::{AudioPlayer, AudioSource, PlaybackSettings},
    prelude::*,
};

#[derive(Message, Clone, Copy)]
pub(crate) enum SoundEffect {
    BrickHit,
    WallHit,
    PaddleHit,
}

#[derive(Resource)]
struct SoundAssets {
    brick_hit: Handle<AudioSource>,
    wall_hit: Handle<AudioSource>,
    paddle_hit: Handle<AudioSource>,
}

impl FromWorld for SoundAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        Self {
            brick_hit: asset_server.load("audio/brick_hit.ogg"),
            wall_hit: asset_server.load("audio/wall_hit.ogg"),
            paddle_hit: asset_server.load("audio/paddle_hit.ogg"),
        }
    }
}

pub(crate) struct SoundEffectsPlugin;

impl Plugin for SoundEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SoundEffect>()
            .init_resource::<SoundAssets>()
            .add_systems(Update, play_sound_effects);
    }
}

fn play_sound_effects(
    mut effects: MessageReader<SoundEffect>,
    sounds: Res<SoundAssets>,
    mut commands: Commands,
) {
    for effect in effects.read() {
        let sound = match effect {
            SoundEffect::BrickHit => sounds.brick_hit.clone(),
            SoundEffect::WallHit => sounds.wall_hit.clone(),
            SoundEffect::PaddleHit => sounds.paddle_hit.clone(),
        };

        commands.spawn((AudioPlayer::new(sound), PlaybackSettings::DESPAWN));
    }
}
