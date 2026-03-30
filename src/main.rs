mod audio;
mod collectibles;
mod collision;
mod components;
mod constants;
mod effects;
mod obstacles;
mod particles;
mod persistence;
mod player;
mod resources;
mod scoring;
mod state;
mod ui;
mod yeti;

use bevy::prelude::*;
use components::{InGame, MainCamera, SkiSound};
use constants::*;
use resources::{GameData, MenuState, PersistentData, Settings, SfxHandles, SpawnTimers};
use state::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(bevy::log::LogPlugin {
                filter: "wgpu=error,naga=warn,bevy_ecs::world=error".into(),
                ..default()
            })
            .set(WindowPlugin {
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
        .init_resource::<PersistentData>()
        .init_resource::<Settings>()
        .init_resource::<MenuState>()
        .add_plugins((
            audio::GameAudioPlugin,
            player::PlayerPlugin,
            obstacles::ObstaclePlugin,
            yeti::YetiPlugin,
            particles::ParticlesPlugin,
            collision::CollisionPlugin,
            scoring::ScoringPlugin,
            effects::EffectsPlugin,
            collectibles::CollectiblesPlugin,
            ui::UIPlugin,
        ))
        .add_systems(Startup, setup_camera)
        .add_systems(
            OnEnter(GameState::Playing),
            (player::cleanup_game, setup_game).chain(),
        )
        // Dying state: freeze + shake, then transition to GameOver
        .add_systems(Update, tick_death_timer.run_if(in_state(GameState::Dying)))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn tick_death_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut game_data: ResMut<GameData>,
    mut next_state: ResMut<NextState<GameState>>,
    sfx: Res<SfxHandles>,
    settings: Res<Settings>,
) {
    game_data.death_timer -= time.delta_seconds();
    if game_data.death_timer <= 0.0 {
        commands.spawn(AudioBundle {
            source: sfx.game_over.clone(),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                volume: bevy::audio::Volume::new(0.5 * settings.effective_sfx()),
                ..default()
            },
        });
        next_state.set(GameState::GameOver);
    }
}

fn setup_game(
    mut commands: Commands,
    mut game_data: ResMut<GameData>,
    mut timers: ResMut<SpawnTimers>,
    sfx: Res<SfxHandles>,
    settings: Res<Settings>,
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
                volume: bevy::audio::Volume::new(0.15 * settings.effective_sfx()),
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
