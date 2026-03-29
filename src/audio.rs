use bevy::prelude::*;
use crate::components::BgMusic;
use crate::resources::SfxHandles;

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_audio);
    }
}

fn setup_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("music.wav"),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Loop,
                volume: bevy::audio::Volume::new(0.35),
                ..default()
            },
        },
        BgMusic,
    ));

    commands.insert_resource(SfxHandles {
        yeti_roar: asset_server.load("yeti_roar.wav"),
        hit: asset_server.load("hit.wav"),
        ski_loop: asset_server.load("ski_loop.wav"),
    });
}
