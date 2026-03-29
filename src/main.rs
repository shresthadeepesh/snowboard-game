mod audio;
mod collision;
mod components;
mod constants;
mod obstacles;
mod particles;
mod player;
mod resources;
mod scoring;
mod state;
mod ui;
mod yeti;

use bevy::prelude::*;
use components::{InGame, SkiSound};
use constants::*;
use resources::{GameData, SfxHandles, SpawnTimers};
use state::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snow Rider - Escape the Mountain!".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(COLOR_SNOW_BG))
        .init_state::<GameState>()
        .init_resource::<GameData>()
        .init_resource::<SpawnTimers>()
        .add_plugins((
            audio::GameAudioPlugin,
            player::PlayerPlugin,
            obstacles::ObstaclePlugin,
            yeti::YetiPlugin,
            particles::ParticlesPlugin,
            collision::CollisionPlugin,
            scoring::ScoringPlugin,
            ui::UIPlugin,
        ))
        .add_systems(Startup, setup_camera)
        .add_systems(
            OnEnter(GameState::Playing),
            (player::cleanup_game, setup_game).chain(),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_game(
    mut commands: Commands,
    mut game_data: ResMut<GameData>,
    mut timers: ResMut<SpawnTimers>,
    sfx: Res<SfxHandles>,
) {
    if game_data.is_paused_resume {
        game_data.is_paused_resume = false;
        return;
    }

    // Start skiing sound
    commands.spawn((
        AudioBundle {
            source: sfx.ski_loop.clone(),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Loop,
                volume: bevy::audio::Volume::new(0.15),
                ..default()
            },
        },
        SkiSound,
        InGame,
    ));

    game_data.reset();
    *timers = SpawnTimers::default();

    player::spawn_background(&mut commands);
    player::spawn_player(&mut commands);
    ui::spawn_hud(&mut commands);
}
