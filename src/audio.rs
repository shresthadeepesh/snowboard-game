use bevy::prelude::*;
use crate::components::BgMusic;
use crate::resources::{Settings, SfxHandles};

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_audio)
            .add_systems(Update, sync_music_volume);
    }
}

fn setup_audio(mut commands: Commands, asset_server: Res<AssetServer>, settings: Res<Settings>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("music.wav"),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Loop,
                volume: bevy::audio::Volume::new(0.35 * settings.effective_music()),
                ..default()
            },
        },
        BgMusic,
    ));

    commands.insert_resource(SfxHandles {
        yeti_roar: asset_server.load("yeti_roar.wav"),
        hit: asset_server.load("hit.wav"),
        ski_loop: asset_server.load("ski_loop.wav"),
        coin: asset_server.load("coin.wav"),
        powerup: asset_server.load("powerup.wav"),
        near_miss: asset_server.load("near_miss.wav"),
        game_over: asset_server.load("game_over.wav"),
    });
}

fn sync_music_volume(
    settings: Res<Settings>,
    music_query: Query<&AudioSink, With<BgMusic>>,
) {
    if !settings.is_changed() {
        return;
    }
    for sink in &music_query {
        sink.set_volume(0.35 * settings.effective_music());
    }
}
