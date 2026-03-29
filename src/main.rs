use bevy::prelude::*;
use rand::Rng;

// ── Constants ──────────────────────────────────────────────────────────────────

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 900.0;
const PLAYER_SPEED: f32 = 400.0;
const BASE_SCROLL_SPEED: f32 = 250.0;
const MAX_SCROLL_SPEED: f32 = 700.0;
const SPEED_INCREASE_RATE: f32 = 8.0;
const OBSTACLE_SPAWN_INTERVAL: f32 = 0.6;
const SNOW_SPAWN_INTERVAL: f32 = 0.02;
const PLAYER_HITBOX_HALF: f32 = 18.0;
const TRAIL_SPAWN_INTERVAL: f32 = 0.03;
const CARVE_LEAN_ANGLE: f32 = 0.35; // max lean angle when carving (~20 deg)
const IDLE_SWAY_SPEED: f32 = 1.8; // how fast idle carving oscillates
const IDLE_SWAY_AMOUNT: f32 = 120.0; // horizontal sway pixels when idle
const IDLE_LEAN_AMOUNT: f32 = 0.18; // lean angle during idle sway
const YETI_SPAWN_DELAY: f32 = 3.0; // seconds before yeti appears
const YETI_CLOSE_SPEED: f32 = 2.0; // slow normal creep toward player
const YETI_CHASE_SPEED_X: f32 = 200.0; // yeti horizontal tracking speed
const YETI_CATCH_RADIUS: f32 = 30.0; // distance at which yeti catches player
const YETI_START_DISTANCE: f32 = 200.0; // initial distance above player (visible on screen)
const YETI_HIT_LUNGE: f32 = 50.0; // how much closer yeti jumps on obstacle hit
const YETI_STUN_CLOSE_SPEED: f32 = 15.0; // yeti close speed while player is stunned
const YETI_MIN_STUN_GAP: f32 = 60.0; // yeti won't auto-catch during stun (stops here)
const YETI_RETREAT_SPEED: f32 = 20.0; // how fast yeti backs off after stun wears off
const YETI_COMFORT_GAP: f32 = 140.0; // gap yeti retreats to after stun
const STUN_DURATION: f32 = 3.0; // seconds player is slowed after hit
const STUN_SLOW_FACTOR: f32 = 0.4; // scroll speed multiplier when stunned
const HIT_COOLDOWN: f32 = 0.5; // brief invincibility after being hit

// ── Colors ─────────────────────────────────────────────────────────────────────

const COLOR_SNOW_BG: Color = Color::srgb(0.92, 0.95, 0.98);
const COLOR_PLAYER_BOARD: Color = Color::srgb(0.15, 0.5, 0.85);
const COLOR_PLAYER_BODY: Color = Color::srgb(0.9, 0.3, 0.2);
const COLOR_PLAYER_HEAD: Color = Color::srgb(0.95, 0.82, 0.7);
const COLOR_TREE_TRUNK: Color = Color::srgb(0.45, 0.3, 0.15);
const COLOR_TREE_LEAVES: Color = Color::srgb(0.15, 0.55, 0.2);
const COLOR_TREE_LEAVES_LIGHT: Color = Color::srgb(0.2, 0.65, 0.3);
const COLOR_TREE_SNOW: Color = Color::srgb(0.95, 0.97, 1.0);
const COLOR_ROCK_DARK: Color = Color::srgb(0.4, 0.42, 0.45);
const COLOR_ROCK_LIGHT: Color = Color::srgb(0.55, 0.57, 0.6);
const COLOR_ROCK_HIGHLIGHT: Color = Color::srgb(0.7, 0.72, 0.75);
const COLOR_DEER_BODY: Color = Color::srgb(0.6, 0.4, 0.2);
const COLOR_DEER_BELLY: Color = Color::srgb(0.85, 0.75, 0.6);
const COLOR_DEER_ANTLER: Color = Color::srgb(0.5, 0.35, 0.15);
const COLOR_RABBIT_BODY: Color = Color::srgb(0.85, 0.82, 0.78);
const COLOR_RABBIT_EAR: Color = Color::srgb(0.95, 0.75, 0.75);
const COLOR_TRAIL: Color = Color::srgba(0.7, 0.82, 0.95, 0.5);
const COLOR_TITLE: Color = Color::srgb(0.1, 0.3, 0.6);

// ── Game States ────────────────────────────────────────────────────────────────

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    Playing,
    Paused,
    GameOver,
}

// ── Components ─────────────────────────────────────────────────────────────────

const PLAYER_TARGET_Y: f32 = -WINDOW_HEIGHT / 8.0;
const PLAYER_INTRO_SPEED: f32 = 200.0; // how fast player slides into position

#[derive(Component)]
struct Player {
    sway_timer: f32, // tracks idle carving oscillation
    intro_done: bool, // whether the slide-in animation is complete
}

#[derive(Component)]
struct PlayerPart;

#[derive(Component)]
struct BackgroundStrip;

#[derive(Component)]
struct Obstacle {
    hitbox_radius: f32,
}

#[derive(Component)]
struct ObstaclePart;

#[derive(Component)]
struct SnowParticle {
    lifetime: f32,
    drift: f32,
}

#[derive(Component)]
struct Trail {
    lifetime: f32,
}

#[derive(Component)]
struct MenuUI;

#[derive(Component)]
struct GameOverUI;

#[derive(Component)]
struct PausedUI;

#[derive(Component)]
struct HudUI;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct SpeedText;

#[derive(Component)]
struct FinalScoreText;

#[derive(Component)]
struct TerrainMarker {
    drift: f32,
}

const YETI_INTRO_SPEED: f32 = 250.0; // how fast yeti slides in from top

#[derive(Component)]
struct Yeti {
    target_y: f32,   // where the yeti should end up after intro
    intro_done: bool, // whether the slide-in is complete
}

#[derive(Component)]
struct YetiPart;

#[derive(Component)]
struct YetiWarningText;

#[derive(Component)]
struct StunnedOverlay;

#[derive(Component)]
struct StunnedText;

#[derive(Component)]
struct BgMusic;

#[derive(Component)]
struct SkiSound;

#[derive(Resource)]
struct SfxHandles {
    yeti_roar: Handle<AudioSource>,
    hit: Handle<AudioSource>,
    ski_loop: Handle<AudioSource>,
}

// ── Resources ──────────────────────────────────────────────────────────────────

#[derive(Resource)]
struct GameData {
    score: f32,
    scroll_speed: f32,
    distance: f32,
    high_score: u32,
    stunned_timer: f32,
    hit_cooldown: f32,
    yeti_lunge_pending: f32,
    is_paused_resume: bool,
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            score: 0.0,
            scroll_speed: BASE_SCROLL_SPEED,
            distance: 0.0,
            high_score: 0,
            stunned_timer: 0.0,
            hit_cooldown: 0.0,
            yeti_lunge_pending: 0.0,
            is_paused_resume: false,
        }
    }
}

#[derive(Resource)]
struct SpawnTimers {
    obstacle_timer: Timer,
    snow_timer: Timer,
    trail_timer: Timer,
    terrain_timer: Timer,
    yeti_timer: Timer,
    yeti_spawned: bool,
}

impl Default for SpawnTimers {
    fn default() -> Self {
        Self {
            obstacle_timer: Timer::from_seconds(OBSTACLE_SPAWN_INTERVAL, TimerMode::Repeating),
            snow_timer: Timer::from_seconds(SNOW_SPAWN_INTERVAL, TimerMode::Repeating),
            trail_timer: Timer::from_seconds(TRAIL_SPAWN_INTERVAL, TimerMode::Repeating),
            terrain_timer: Timer::from_seconds(0.15, TimerMode::Repeating),
            yeti_timer: Timer::from_seconds(YETI_SPAWN_DELAY, TimerMode::Once),
            yeti_spawned: false,
        }
    }
}

// ── Main ───────────────────────────────────────────────────────────────────────

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
        // Menu
        .add_systems(OnEnter(GameState::Menu), (cleanup_game, setup_menu).chain())
        .add_systems(OnExit(GameState::Menu), cleanup::<MenuUI>)
        .add_systems(Update, menu_input.run_if(in_state(GameState::Menu)))
        // Playing
        .add_systems(OnEnter(GameState::Playing), (cleanup_game, setup_game).chain())
        .add_systems(
            Update,
            (
                player_movement,
                scroll_obstacles,
                spawn_obstacles,
                spawn_snow,
                spawn_trail,
                spawn_terrain_marks,
                update_snow,
                update_trails,
                update_terrain_marks,
                check_collisions,
                spawn_yeti,
                update_yeti,
                update_stun,
                update_score,
                update_hud,
                pause_input,
            )
                .run_if(in_state(GameState::Playing)),
        )
        // Paused
        .add_systems(OnEnter(GameState::Paused), setup_paused)
        .add_systems(OnExit(GameState::Paused), cleanup::<PausedUI>)
        .add_systems(Update, paused_input.run_if(in_state(GameState::Paused)))
        // Game Over
        .add_systems(OnEnter(GameState::GameOver), (cleanup_game, setup_game_over).chain())
        .add_systems(OnExit(GameState::GameOver), cleanup::<GameOverUI>)
        .add_systems(Update, game_over_input.run_if(in_state(GameState::GameOver)))
        // Global
        .add_systems(Startup, (setup_camera, setup_music))
        .run();
}

// ── Camera ─────────────────────────────────────────────────────────────────────

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// ── Music ──────────────────────────────────────────────────────────────────────

fn setup_music(mut commands: Commands, asset_server: Res<AssetServer>) {
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

// ── Menu ───────────────────────────────────────────────────────────────────────

fn setup_menu(mut commands: Commands, game_data: Res<GameData>) {
    // Background overlay
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.85, 0.9, 0.95, 1.0),
                custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, -1.0),
            ..default()
        },
        MenuUI,
    ));

    // Decorative mountains
    for (x, w, h, c) in [
        (-200.0, 350.0, 250.0, Color::srgb(0.75, 0.8, 0.88)),
        (0.0, 400.0, 320.0, Color::srgb(0.7, 0.76, 0.85)),
        (200.0, 300.0, 200.0, Color::srgb(0.78, 0.83, 0.9)),
    ] {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: c,
                    custom_size: Some(Vec2::new(w, h)),
                    ..default()
                },
                transform: Transform::from_xyz(x, -150.0, 0.0),
                ..default()
            },
            MenuUI,
        ));
    }

    // Snow cap on mountains
    for (x, w) in [(-200.0, 200.0), (0.0, 250.0), (200.0, 180.0)] {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(w, 30.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, -30.0, 0.1),
                ..default()
            },
            MenuUI,
        ));
    }

    // Title
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "SNOW RIDER",
                TextStyle {
                    font_size: 72.0,
                    color: COLOR_TITLE,
                    ..default()
                },
            )
            .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, 250.0, 1.0),
            ..default()
        },
        MenuUI,
    ));

    // Subtitle
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Escape the Mountain!",
                TextStyle {
                    font_size: 28.0,
                    color: Color::srgb(0.3, 0.45, 0.65),
                    ..default()
                },
            )
            .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, 190.0, 1.0),
            ..default()
        },
        MenuUI,
    ));

    // Decorative snowboarder silhouette (big)
    spawn_menu_snowboarder(&mut commands, Vec3::new(0.0, 50.0, 1.0), 2.5);

    // Instructions
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Press SPACE or ENTER to Start",
                TextStyle {
                    font_size: 30.0,
                    color: Color::srgb(0.2, 0.35, 0.55),
                    ..default()
                },
            )
            .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, -120.0, 1.0),
            ..default()
        },
        MenuUI,
    ));

    // Controls info
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Arrow Keys / A,D to Move  |  ESC to Pause",
                TextStyle {
                    font_size: 18.0,
                    color: Color::srgb(0.4, 0.5, 0.65),
                    ..default()
                },
            )
            .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, -170.0, 1.0),
            ..default()
        },
        MenuUI,
    ));

    // High score
    if game_data.high_score > 0 {
        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    format!("High Score: {}", game_data.high_score),
                    TextStyle {
                        font_size: 24.0,
                        color: Color::srgb(0.8, 0.6, 0.1),
                        ..default()
                    },
                )
                .with_justify(JustifyText::Center),
                transform: Transform::from_xyz(0.0, -220.0, 1.0),
                ..default()
            },
            MenuUI,
        ));
    }
}

fn spawn_menu_snowboarder(commands: &mut Commands, pos: Vec3, scale: f32) {
    // Board
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: COLOR_PLAYER_BOARD,
                custom_size: Some(Vec2::new(50.0 * scale, 8.0 * scale)),
                ..default()
            },
            transform: Transform::from_translation(pos + Vec3::new(0.0, -20.0 * scale, 0.0))
                .with_rotation(Quat::from_rotation_z(-0.25)),
            ..default()
        },
        MenuUI,
    ));
    // Body
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: COLOR_PLAYER_BODY,
                custom_size: Some(Vec2::new(18.0 * scale, 30.0 * scale)),
                ..default()
            },
            transform: Transform::from_translation(pos + Vec3::new(0.0, 5.0 * scale, 0.1)),
            ..default()
        },
        MenuUI,
    ));
    // Head
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: COLOR_PLAYER_HEAD,
                custom_size: Some(Vec2::new(14.0 * scale, 14.0 * scale)),
                ..default()
            },
            transform: Transform::from_translation(pos + Vec3::new(0.0, 28.0 * scale, 0.1)),
            ..default()
        },
        MenuUI,
    ));
}

fn menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::Enter) {
        next_state.set(GameState::Playing);
    }
}

// ── Game Setup ─────────────────────────────────────────────────────────────────

fn setup_game(
    mut commands: Commands,
    mut game_data: ResMut<GameData>,
    mut timers: ResMut<SpawnTimers>,
    sfx: Res<SfxHandles>,
) {
    // If resuming from pause, skip all setup
    if game_data.is_paused_resume {
        game_data.is_paused_resume = false;
        return;
    }

    // Start skiing sound loop
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
    ));

    // Reset game data
    game_data.score = 0.0;
    game_data.scroll_speed = BASE_SCROLL_SPEED;
    game_data.distance = 0.0;
    game_data.stunned_timer = 0.0;
    game_data.hit_cooldown = 0.0;
    game_data.yeti_lunge_pending = 0.0;

    // Reset timers
    *timers = SpawnTimers::default();

    // Snow ground background strips for depth
    for i in 0..20 {
        let y = -WINDOW_HEIGHT / 2.0 + i as f32 * 50.0;
        let shade = 0.88 + (i as f32 * 0.005);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(shade, shade + 0.02, shade + 0.05),
                    custom_size: Some(Vec2::new(WINDOW_WIDTH, 50.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, y, -0.5),
                ..default()
            },
            BackgroundStrip,
        ));
    }

    // Spawn player (composite entity with children)
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::NONE,
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, WINDOW_HEIGHT / 2.0 + 60.0, 10.0),
                ..default()
            },
            Player { sway_timer: 0.0, intro_done: false },
        ))
        .with_children(|parent| {
            // Shadow
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(0.0, 0.0, 0.0, 0.15),
                        custom_size: Some(Vec2::new(48.0, 12.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, -28.0, -0.1),
                    ..default()
                },
                PlayerPart,
            ));
            // Snowboard
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_PLAYER_BOARD,
                        custom_size: Some(Vec2::new(50.0, 8.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, -22.0, 0.0),
                    ..default()
                },
                PlayerPart,
            ));
            // Board edge highlights
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.1, 0.35, 0.7),
                        custom_size: Some(Vec2::new(52.0, 2.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, -18.0, 0.01),
                    ..default()
                },
                PlayerPart,
            ));
            // Legs
            for x in [-6.0, 6.0] {
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgb(0.2, 0.25, 0.35),
                            custom_size: Some(Vec2::new(7.0, 14.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(x, -10.0, 0.1),
                        ..default()
                    },
                    PlayerPart,
                ));
            }
            // Body / jacket
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_PLAYER_BODY,
                        custom_size: Some(Vec2::new(22.0, 26.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 8.0, 0.2),
                    ..default()
                },
                PlayerPart,
            ));
            // Jacket stripe
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.95, 0.85, 0.2),
                        custom_size: Some(Vec2::new(22.0, 4.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 4.0, 0.25),
                    ..default()
                },
                PlayerPart,
            ));
            // Arms
            for x in [-14.0, 14.0] {
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: COLOR_PLAYER_BODY,
                            custom_size: Some(Vec2::new(8.0, 6.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(x, 12.0, 0.2),
                        ..default()
                    },
                    PlayerPart,
                ));
            }
            // Head
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_PLAYER_HEAD,
                        custom_size: Some(Vec2::new(16.0, 16.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 28.0, 0.3),
                    ..default()
                },
                PlayerPart,
            ));
            // Goggles
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.1, 0.1, 0.15),
                        custom_size: Some(Vec2::new(14.0, 5.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 30.0, 0.35),
                    ..default()
                },
                PlayerPart,
            ));
            // Goggle lens
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.3, 0.7, 0.95),
                        custom_size: Some(Vec2::new(10.0, 3.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 30.0, 0.36),
                    ..default()
                },
                PlayerPart,
            ));
            // Beanie hat
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.8, 0.2, 0.15),
                        custom_size: Some(Vec2::new(18.0, 10.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 38.0, 0.3),
                    ..default()
                },
                PlayerPart,
            ));
            // Hat pom-pom
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::WHITE,
                        custom_size: Some(Vec2::new(8.0, 8.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 44.0, 0.35),
                    ..default()
                },
                PlayerPart,
            ));
        });

    // Score text
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Score: 0",
                TextStyle {
                    font_size: 28.0,
                    color: Color::srgb(0.1, 0.1, 0.2),
                    ..default()
                },
            )
            .with_justify(JustifyText::Left),
            transform: Transform::from_xyz(-WINDOW_WIDTH / 2.0 + 20.0, WINDOW_HEIGHT / 2.0 - 25.0, 51.0),
            text_anchor: bevy::sprite::Anchor::CenterLeft,
            ..default()
        },
        HudUI,
        ScoreText,
    ));

    // Speed text
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Speed: 100%",
                TextStyle {
                    font_size: 28.0,
                    color: Color::srgb(0.1, 0.1, 0.2),
                    ..default()
                },
            )
            .with_justify(JustifyText::Right),
            transform: Transform::from_xyz(WINDOW_WIDTH / 2.0 - 20.0, WINDOW_HEIGHT / 2.0 - 25.0, 51.0),
            text_anchor: bevy::sprite::Anchor::CenterRight,
            ..default()
        },
        HudUI,
        SpeedText,
    ));
}

// ── Cleanup ────────────────────────────────────────────────────────────────────

fn cleanup<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn cleanup_game(
    mut commands: Commands,
    game_data: Res<GameData>,
    players: Query<Entity, With<Player>>,
    bg: Query<Entity, With<BackgroundStrip>>,
    obstacles: Query<Entity, With<Obstacle>>,
    snow: Query<Entity, With<SnowParticle>>,
    trails: Query<Entity, With<Trail>>,
    hud: Query<Entity, With<HudUI>>,
    terrain: Query<Entity, With<TerrainMarker>>,
    yetis: Query<Entity, With<Yeti>>,
    yeti_warnings: Query<Entity, With<YetiWarningText>>,
    stun_overlays: Query<Entity, With<StunnedOverlay>>,
    stun_texts: Query<Entity, With<StunnedText>>,
    ski_sounds: Query<Entity, With<SkiSound>>,
) {
    // Skip cleanup when resuming from pause
    if game_data.is_paused_resume {
        return;
    }

    for entity in players.iter().chain(bg.iter()).chain(obstacles.iter()).chain(snow.iter()).chain(trails.iter()).chain(hud.iter()).chain(terrain.iter()).chain(yetis.iter()).chain(yeti_warnings.iter()).chain(stun_overlays.iter()).chain(stun_texts.iter()).chain(ski_sounds.iter()) {
        commands.entity(entity).despawn_recursive();
    }
}

// ── Player Movement ────────────────────────────────────────────────────────────

fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    let dt = time.delta_seconds();

    for (mut transform, mut player) in &mut query {
        // Advance the sway timer (always ticks for idle carving)
        player.sway_timer += dt * IDLE_SWAY_SPEED;

        // Intro slide: player enters from top of screen while fully controllable
        if !player.intro_done {
            transform.translation.y -= PLAYER_INTRO_SPEED * dt;
            if transform.translation.y <= PLAYER_TARGET_Y {
                transform.translation.y = PLAYER_TARGET_Y;
                player.intro_done = true;
            }
        }

        let mut direction: f32 = 0.0;
        if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
            direction -= 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
            direction += 1.0;
        }

        let is_turning = direction.abs() > 0.1;

        if is_turning {
            // Active carving: move in input direction with strong lean
            transform.translation.x += direction * PLAYER_SPEED * dt;

            let target_lean = -direction * CARVE_LEAN_ANGLE;
            let current = transform.rotation.to_euler(EulerRot::XYZ).2;
            let new_rot = current + (target_lean - current) * 10.0 * dt;
            transform.rotation = Quat::from_rotation_z(new_rot);

            // Sync sway timer to current direction so idle resumes smoothly
            if direction > 0.0 {
                player.sway_timer = std::f32::consts::FRAC_PI_2; // peak right
            } else {
                player.sway_timer = -std::f32::consts::FRAC_PI_2; // peak left
            }
        } else {
            // Idle carving: gentle S-curve sway left and right
            let sway = player.sway_timer.sin();

            // Horizontal sway movement
            transform.translation.x += sway * IDLE_SWAY_AMOUNT * dt;

            // Lean into the sway direction
            let target_lean = -sway * IDLE_LEAN_AMOUNT;
            let current = transform.rotation.to_euler(EulerRot::XYZ).2;
            let new_rot = current + (target_lean - current) * 6.0 * dt;
            transform.rotation = Quat::from_rotation_z(new_rot);
        }

        // Clamp to window bounds
        let half_width = WINDOW_WIDTH / 2.0 - 30.0;
        transform.translation.x = transform.translation.x.clamp(-half_width, half_width);
    }
}

fn pause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
    }
    if keyboard.just_pressed(KeyCode::KeyR) {
        next_state.set(GameState::Playing);
    }
}

// ── Pause Screen ──────────────────────────────────────────────────────────────

fn setup_paused(mut commands: Commands) {
    // Dark overlay
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.0, 0.0, 0.05, 0.7),
                custom_size: Some(Vec2::new(WINDOW_WIDTH * 2.0, WINDOW_HEIGHT * 2.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            ..default()
        },
        PausedUI,
    ));

    // Panel background
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.08, 0.1, 0.22),
                custom_size: Some(Vec2::new(380.0, 280.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 20.0, 101.0),
            ..default()
        },
        PausedUI,
    ));

    // Panel border
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.25, 0.4, 0.7),
                custom_size: Some(Vec2::new(386.0, 286.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 20.0, 100.5),
            ..default()
        },
        PausedUI,
    ));

    // Title
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "PAUSED",
                TextStyle {
                    font_size: 52.0,
                    color: Color::srgb(1.0, 1.0, 1.0),
                    ..default()
                },
            )
            .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, 100.0, 102.0),
            ..default()
        },
        PausedUI,
    ));

    // Resume
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "ESC  -  Resume",
                TextStyle {
                    font_size: 26.0,
                    color: Color::srgb(0.75, 0.85, 0.95),
                    ..default()
                },
            )
            .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, 30.0, 102.0),
            ..default()
        },
        PausedUI,
    ));

    // Restart
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "R  -  Restart",
                TextStyle {
                    font_size: 26.0,
                    color: Color::srgb(0.75, 0.85, 0.95),
                    ..default()
                },
            )
            .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, -10.0, 102.0),
            ..default()
        },
        PausedUI,
    ));

    // Quit to menu
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Q  -  Quit to Menu",
                TextStyle {
                    font_size: 26.0,
                    color: Color::srgb(0.75, 0.85, 0.95),
                    ..default()
                },
            )
            .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, -50.0, 102.0),
            ..default()
        },
        PausedUI,
    ));
}

fn paused_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut game_data: ResMut<GameData>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        // Resume
        game_data.is_paused_resume = true;
        next_state.set(GameState::Playing);
    }
    if keyboard.just_pressed(KeyCode::KeyR) {
        // Restart (fresh game)
        next_state.set(GameState::Playing);
    }
    if keyboard.just_pressed(KeyCode::KeyQ) {
        // Quit to menu
        next_state.set(GameState::Menu);
    }
}

// ── Obstacle Spawning ──────────────────────────────────────────────────────────

fn spawn_obstacles(
    mut commands: Commands,
    time: Res<Time>,
    mut timers: ResMut<SpawnTimers>,
    game_data: Res<GameData>,
    player_query: Query<&Player>,
) {
    // Don't spawn obstacles during player intro
    if player_query.get_single().map_or(true, |p| !p.intro_done) {
        return;
    }

    timers.obstacle_timer.tick(time.delta());

    if !timers.obstacle_timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-WINDOW_WIDTH / 2.0 + 40.0..WINDOW_WIDTH / 2.0 - 40.0);
    let y = -WINDOW_HEIGHT / 2.0 - 80.0;

    // Difficulty scaling: more obstacle types at higher speeds
    let obstacle_type = if game_data.scroll_speed > 500.0 {
        rng.gen_range(0..5)
    } else if game_data.scroll_speed > 350.0 {
        rng.gen_range(0..4)
    } else {
        rng.gen_range(0..3)
    };

    match obstacle_type {
        0 => spawn_tree(&mut commands, Vec3::new(x, y, 5.0), &mut rng),
        1 => spawn_rock(&mut commands, Vec3::new(x, y, 5.0), &mut rng),
        2 => spawn_deer(&mut commands, Vec3::new(x, y, 5.0)),
        3 => spawn_rabbit(&mut commands, Vec3::new(x, y, 5.0)),
        _ => spawn_big_rock(&mut commands, Vec3::new(x, y, 5.0), &mut rng),
    }
}

fn spawn_tree(commands: &mut Commands, pos: Vec3, rng: &mut impl Rng) {
    let scale = rng.gen_range(0.8..1.3);
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::NONE,
                    ..default()
                },
                transform: Transform::from_translation(pos),
                ..default()
            },
            Obstacle { hitbox_radius: 15.0 * scale },
        ))
        .with_children(|parent| {
            // Shadow
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(0.0, 0.0, 0.0, 0.12),
                        custom_size: Some(Vec2::new(40.0 * scale, 14.0 * scale)),
                        ..default()
                    },
                    transform: Transform::from_xyz(4.0, -20.0 * scale, -0.1),
                    ..default()
                },
                ObstaclePart,
            ));
            // Trunk
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_TREE_TRUNK,
                        custom_size: Some(Vec2::new(10.0 * scale, 28.0 * scale)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..default()
                },
                ObstaclePart,
            ));
            // Bottom foliage (largest)
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_TREE_LEAVES,
                        custom_size: Some(Vec2::new(40.0 * scale, 22.0 * scale)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 14.0 * scale, 0.1),
                    ..default()
                },
                ObstaclePart,
            ));
            // Middle foliage
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_TREE_LEAVES_LIGHT,
                        custom_size: Some(Vec2::new(30.0 * scale, 20.0 * scale)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 28.0 * scale, 0.2),
                    ..default()
                },
                ObstaclePart,
            ));
            // Top foliage
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_TREE_LEAVES,
                        custom_size: Some(Vec2::new(20.0 * scale, 18.0 * scale)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 42.0 * scale, 0.3),
                    ..default()
                },
                ObstaclePart,
            ));
            // Snow on top
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_TREE_SNOW,
                        custom_size: Some(Vec2::new(22.0 * scale, 6.0 * scale)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 48.0 * scale, 0.4),
                    ..default()
                },
                ObstaclePart,
            ));
            // Snow patches on branches
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_TREE_SNOW,
                        custom_size: Some(Vec2::new(28.0 * scale, 4.0 * scale)),
                        ..default()
                    },
                    transform: Transform::from_xyz(2.0, 24.0 * scale, 0.35),
                    ..default()
                },
                ObstaclePart,
            ));
        });
}

fn spawn_rock(commands: &mut Commands, pos: Vec3, rng: &mut impl Rng) {
    let scale = rng.gen_range(0.7..1.4);
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::NONE,
                    ..default()
                },
                transform: Transform::from_translation(pos),
                ..default()
            },
            Obstacle { hitbox_radius: 18.0 * scale },
        ))
        .with_children(|parent| {
            // Shadow
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(0.0, 0.0, 0.0, 0.12),
                        custom_size: Some(Vec2::new(38.0 * scale, 12.0 * scale)),
                        ..default()
                    },
                    transform: Transform::from_xyz(3.0, -12.0 * scale, -0.1),
                    ..default()
                },
                ObstaclePart,
            ));
            // Main rock body
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_ROCK_DARK,
                        custom_size: Some(Vec2::new(34.0 * scale, 24.0 * scale)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..default()
                },
                ObstaclePart,
            ));
            // Rock highlight
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_ROCK_LIGHT,
                        custom_size: Some(Vec2::new(26.0 * scale, 16.0 * scale)),
                        ..default()
                    },
                    transform: Transform::from_xyz(-2.0, 3.0, 0.1),
                    ..default()
                },
                ObstaclePart,
            ));
            // Top highlight
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_ROCK_HIGHLIGHT,
                        custom_size: Some(Vec2::new(14.0 * scale, 8.0 * scale)),
                        ..default()
                    },
                    transform: Transform::from_xyz(-3.0, 6.0 * scale, 0.2),
                    ..default()
                },
                ObstaclePart,
            ));
            // Snow dusting
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_TREE_SNOW,
                        custom_size: Some(Vec2::new(20.0 * scale, 4.0 * scale)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 12.0 * scale, 0.3),
                    ..default()
                },
                ObstaclePart,
            ));
        });
}

fn spawn_big_rock(commands: &mut Commands, pos: Vec3, rng: &mut impl Rng) {
    let scale = rng.gen_range(1.2..1.8);
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::NONE,
                    ..default()
                },
                transform: Transform::from_translation(pos),
                ..default()
            },
            Obstacle { hitbox_radius: 22.0 * scale },
        ))
        .with_children(|parent| {
            // Shadow
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(0.0, 0.0, 0.0, 0.15),
                        custom_size: Some(Vec2::new(50.0 * scale, 16.0 * scale)),
                        ..default()
                    },
                    transform: Transform::from_xyz(4.0, -16.0 * scale, -0.1),
                    ..default()
                },
                ObstaclePart,
            ));
            // Base rock
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.35, 0.37, 0.4),
                        custom_size: Some(Vec2::new(44.0 * scale, 30.0 * scale)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..default()
                },
                ObstaclePart,
            ));
            // Lighter area
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_ROCK_DARK,
                        custom_size: Some(Vec2::new(36.0 * scale, 22.0 * scale)),
                        ..default()
                    },
                    transform: Transform::from_xyz(-2.0, 2.0, 0.1),
                    ..default()
                },
                ObstaclePart,
            ));
            // Highlight
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_ROCK_HIGHLIGHT,
                        custom_size: Some(Vec2::new(18.0 * scale, 10.0 * scale)),
                        ..default()
                    },
                    transform: Transform::from_xyz(-4.0, 8.0 * scale, 0.2),
                    ..default()
                },
                ObstaclePart,
            ));
            // Snow on top
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_TREE_SNOW,
                        custom_size: Some(Vec2::new(30.0 * scale, 6.0 * scale)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 14.0 * scale, 0.3),
                    ..default()
                },
                ObstaclePart,
            ));
        });
}

fn spawn_deer(commands: &mut Commands, pos: Vec3) {
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::NONE,
                    ..default()
                },
                transform: Transform::from_translation(pos),
                ..default()
            },
            Obstacle { hitbox_radius: 20.0 },
        ))
        .with_children(|parent| {
            // Shadow
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(0.0, 0.0, 0.0, 0.1),
                        custom_size: Some(Vec2::new(40.0, 10.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, -20.0, -0.1),
                    ..default()
                },
                ObstaclePart,
            ));
            // Legs
            for x in [-10.0, -4.0, 4.0, 10.0] {
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgb(0.5, 0.33, 0.15),
                            custom_size: Some(Vec2::new(4.0, 18.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(x, -10.0, 0.0),
                        ..default()
                    },
                    ObstaclePart,
                ));
            }
            // Body
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_DEER_BODY,
                        custom_size: Some(Vec2::new(36.0, 20.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 4.0, 0.1),
                    ..default()
                },
                ObstaclePart,
            ));
            // Belly
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_DEER_BELLY,
                        custom_size: Some(Vec2::new(28.0, 10.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, -1.0, 0.15),
                    ..default()
                },
                ObstaclePart,
            ));
            // Neck
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_DEER_BODY,
                        custom_size: Some(Vec2::new(10.0, 16.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(14.0, 16.0, 0.2),
                    ..default()
                },
                ObstaclePart,
            ));
            // Head
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_DEER_BODY,
                        custom_size: Some(Vec2::new(14.0, 12.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(18.0, 26.0, 0.3),
                    ..default()
                },
                ObstaclePart,
            ));
            // Nose
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.15, 0.12, 0.1),
                        custom_size: Some(Vec2::new(4.0, 4.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(24.0, 24.0, 0.35),
                    ..default()
                },
                ObstaclePart,
            ));
            // Eye
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.1, 0.08, 0.06),
                        custom_size: Some(Vec2::new(3.0, 3.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(20.0, 28.0, 0.35),
                    ..default()
                },
                ObstaclePart,
            ));
            // Antlers
            for (dx, dy) in [(16.0, 34.0), (20.0, 38.0), (14.0, 40.0)] {
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: COLOR_DEER_ANTLER,
                            custom_size: Some(Vec2::new(3.0, 10.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(dx, dy, 0.4),
                        ..default()
                    },
                    ObstaclePart,
                ));
            }
            // Tail
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_DEER_BELLY,
                        custom_size: Some(Vec2::new(6.0, 6.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(-18.0, 10.0, 0.15),
                    ..default()
                },
                ObstaclePart,
            ));
        });
}

fn spawn_rabbit(commands: &mut Commands, pos: Vec3) {
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::NONE,
                    ..default()
                },
                transform: Transform::from_translation(pos),
                ..default()
            },
            Obstacle { hitbox_radius: 12.0 },
        ))
        .with_children(|parent| {
            // Shadow
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(0.0, 0.0, 0.0, 0.08),
                        custom_size: Some(Vec2::new(20.0, 8.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, -12.0, -0.1),
                    ..default()
                },
                ObstaclePart,
            ));
            // Body
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_RABBIT_BODY,
                        custom_size: Some(Vec2::new(20.0, 16.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, -2.0, 0.1),
                    ..default()
                },
                ObstaclePart,
            ));
            // Head
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_RABBIT_BODY,
                        custom_size: Some(Vec2::new(14.0, 12.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(6.0, 8.0, 0.2),
                    ..default()
                },
                ObstaclePart,
            ));
            // Ears
            for dx in [3.0, 9.0] {
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: COLOR_RABBIT_BODY,
                            custom_size: Some(Vec2::new(5.0, 14.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(dx, 20.0, 0.3),
                        ..default()
                    },
                    ObstaclePart,
                ));
                // Inner ear
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: COLOR_RABBIT_EAR,
                            custom_size: Some(Vec2::new(3.0, 10.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(dx, 20.0, 0.35),
                        ..default()
                    },
                    ObstaclePart,
                ));
            }
            // Eye
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.15, 0.1, 0.1),
                        custom_size: Some(Vec2::new(3.0, 3.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(10.0, 10.0, 0.35),
                    ..default()
                },
                ObstaclePart,
            ));
            // Nose
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.9, 0.6, 0.6),
                        custom_size: Some(Vec2::new(2.0, 2.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(12.0, 7.0, 0.35),
                    ..default()
                },
                ObstaclePart,
            ));
            // Fluffy tail
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::WHITE,
                        custom_size: Some(Vec2::new(8.0, 8.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(-10.0, 0.0, 0.15),
                    ..default()
                },
                ObstaclePart,
            ));
            // Feet
            for x in [-4.0, 4.0] {
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgb(0.78, 0.75, 0.72),
                            custom_size: Some(Vec2::new(6.0, 5.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(x, -10.0, 0.05),
                        ..default()
                    },
                    ObstaclePart,
                ));
            }
        });
}

// ── Scrolling ──────────────────────────────────────────────────────────────────

fn scroll_obstacles(
    mut commands: Commands,
    time: Res<Time>,
    game_data: Res<GameData>,
    mut query: Query<(Entity, &mut Transform), With<Obstacle>>,
) {
    let speed_mult = if game_data.stunned_timer > 0.0 { STUN_SLOW_FACTOR } else { 1.0 };
    for (entity, mut transform) in &mut query {
        transform.translation.y += game_data.scroll_speed * speed_mult * time.delta_seconds();
        if transform.translation.y > WINDOW_HEIGHT / 2.0 + 100.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// ── Snow Particles ─────────────────────────────────────────────────────────────

fn spawn_snow(
    mut commands: Commands,
    time: Res<Time>,
    mut timers: ResMut<SpawnTimers>,
) {
    timers.snow_timer.tick(time.delta());

    if !timers.snow_timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-WINDOW_WIDTH / 2.0..WINDOW_WIDTH / 2.0);
    let size = rng.gen_range(2.0..6.0);
    let drift = rng.gen_range(-30.0..30.0);
    let alpha = rng.gen_range(0.3..0.8);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(1.0, 1.0, 1.0, alpha),
                custom_size: Some(Vec2::new(size, size)),
                ..default()
            },
            transform: Transform::from_xyz(x, -WINDOW_HEIGHT / 2.0 - 10.0, 20.0),
            ..default()
        },
        SnowParticle {
            lifetime: rng.gen_range(2.0..4.0),
            drift,
        },
    ));
}

fn update_snow(
    mut commands: Commands,
    time: Res<Time>,
    game_data: Res<GameData>,
    mut query: Query<(Entity, &mut Transform, &mut SnowParticle, &mut Sprite)>,
) {
    let speed_mult = if game_data.stunned_timer > 0.0 { STUN_SLOW_FACTOR } else { 1.0 };
    for (entity, mut transform, mut particle, mut sprite) in &mut query {
        // Move upward (simulating downhill motion) and drift sideways
        transform.translation.y += game_data.scroll_speed * speed_mult * 0.8 * time.delta_seconds();
        transform.translation.x += particle.drift * time.delta_seconds();

        particle.lifetime -= time.delta_seconds();

        // Fade out
        if particle.lifetime < 0.5 {
            let a = (particle.lifetime / 0.5).max(0.0);
            sprite.color = Color::srgba(1.0, 1.0, 1.0, a * 0.6);
        }

        if particle.lifetime <= 0.0 || transform.translation.y > WINDOW_HEIGHT / 2.0 + 20.0 {
            commands.entity(entity).despawn();
        }
    }
}

// ── Trail Effect ───────────────────────────────────────────────────────────────

fn spawn_trail(
    mut commands: Commands,
    time: Res<Time>,
    mut timers: ResMut<SpawnTimers>,
    player_query: Query<&Transform, With<Player>>,
) {
    timers.trail_timer.tick(time.delta());

    if !timers.trail_timer.just_finished() {
        return;
    }

    if let Ok(transform) = player_query.get_single() {
        let mut rng = rand::thread_rng();
        let offset_x = rng.gen_range(-8.0..8.0);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: COLOR_TRAIL,
                    custom_size: Some(Vec2::new(12.0, 6.0)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    transform.translation.x + offset_x,
                    transform.translation.y - 35.0,
                    4.0,
                ),
                ..default()
            },
            Trail { lifetime: 0.8 },
        ));
    }
}

fn update_trails(
    mut commands: Commands,
    time: Res<Time>,
    game_data: Res<GameData>,
    mut query: Query<(Entity, &mut Transform, &mut Trail, &mut Sprite)>,
) {
    for (entity, mut transform, mut trail, mut sprite) in &mut query {
        transform.translation.y += game_data.scroll_speed * 0.5 * time.delta_seconds();
        trail.lifetime -= time.delta_seconds();

        let alpha = (trail.lifetime / 0.8).max(0.0) * 0.5;
        sprite.color = Color::srgba(0.7, 0.82, 0.95, alpha);

        // Expand trail as it fades
        if let Some(ref mut size) = sprite.custom_size {
            size.x += 15.0 * time.delta_seconds();
        }

        if trail.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

// ── Terrain Marks (ski tracks, snow patches) ───────────────────────────────────

fn spawn_terrain_marks(
    mut commands: Commands,
    time: Res<Time>,
    mut timers: ResMut<SpawnTimers>,
) {
    timers.terrain_timer.tick(time.delta());

    if !timers.terrain_timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();

    // Random snow shimmer patches
    let x = rng.gen_range(-WINDOW_WIDTH / 2.0..WINDOW_WIDTH / 2.0);
    let w = rng.gen_range(20.0..80.0);
    let h = rng.gen_range(4.0..12.0);
    let alpha = rng.gen_range(0.03..0.08);
    let drift = rng.gen_range(-10.0..10.0);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.8, 0.85, 0.95, alpha),
                custom_size: Some(Vec2::new(w, h)),
                ..default()
            },
            transform: Transform::from_xyz(x, -WINDOW_HEIGHT / 2.0 - 20.0, 0.0),
            ..default()
        },
        TerrainMarker { drift },
    ));
}

fn update_terrain_marks(
    mut commands: Commands,
    time: Res<Time>,
    game_data: Res<GameData>,
    mut query: Query<(Entity, &mut Transform, &TerrainMarker)>,
) {
    for (entity, mut transform, marker) in &mut query {
        transform.translation.y += game_data.scroll_speed * time.delta_seconds();
        transform.translation.x += marker.drift * time.delta_seconds();

        if transform.translation.y > WINDOW_HEIGHT / 2.0 + 30.0 {
            commands.entity(entity).despawn();
        }
    }
}

// ── Yeti ───────────────────────────────────────────────────────────────────────

fn spawn_yeti(
    mut commands: Commands,
    time: Res<Time>,
    mut timers: ResMut<SpawnTimers>,
    player_query: Query<(&Transform, &Player)>,
    sfx: Res<SfxHandles>,
) {
    if timers.yeti_spawned {
        return;
    }

    // Don't start yeti timer until player intro is done
    if player_query.get_single().map_or(true, |(_, p)| !p.intro_done) {
        return;
    }

    timers.yeti_timer.tick(time.delta());

    if !timers.yeti_timer.just_finished() {
        return;
    }

    timers.yeti_spawned = true;

    // Play yeti roar
    commands.spawn(AudioBundle {
        source: sfx.yeti_roar.clone(),
        settings: PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Despawn,
            volume: bevy::audio::Volume::new(0.6),
            ..default()
        },
    });

    // Spawn above the screen, slide down to chase position
    let player_pos = player_query
        .get_single()
        .map(|(t, _)| t.translation)
        .unwrap_or(Vec3::ZERO);
    let target_y = player_pos.y + YETI_START_DISTANCE;
    let spawn_y = WINDOW_HEIGHT / 2.0 + 80.0; // start off-screen above

    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::NONE,
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    ..default()
                },
                transform: Transform::from_xyz(player_pos.x, spawn_y, 15.0),
                ..default()
            },
            Yeti { target_y, intro_done: false },
        ))
        .with_children(|parent| {
            // Shadow
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(0.0, 0.0, 0.0, 0.2),
                        custom_size: Some(Vec2::new(70.0, 20.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, -40.0, -0.1),
                    ..default()
                },
                YetiPart,
            ));
            // Left foot
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.65, 0.62, 0.58),
                        custom_size: Some(Vec2::new(18.0, 14.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(-14.0, -34.0, 0.0),
                    ..default()
                },
                YetiPart,
            ));
            // Right foot
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.65, 0.62, 0.58),
                        custom_size: Some(Vec2::new(18.0, 14.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(14.0, -34.0, 0.0),
                    ..default()
                },
                YetiPart,
            ));
            // Legs
            for x in [-12.0, 12.0] {
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgb(0.88, 0.86, 0.84),
                            custom_size: Some(Vec2::new(16.0, 20.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(x, -22.0, 0.05),
                        ..default()
                    },
                    YetiPart,
                ));
            }
            // Main body (large and bulky)
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.92, 0.9, 0.88),
                        custom_size: Some(Vec2::new(56.0, 50.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 4.0, 0.1),
                    ..default()
                },
                YetiPart,
            ));
            // Belly fur patch
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.96, 0.95, 0.93),
                        custom_size: Some(Vec2::new(36.0, 30.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, -2.0, 0.15),
                    ..default()
                },
                YetiPart,
            ));
            // Left arm (reaching out)
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.88, 0.86, 0.84),
                        custom_size: Some(Vec2::new(16.0, 36.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(-34.0, 8.0, 0.2)
                        .with_rotation(Quat::from_rotation_z(0.3)),
                    ..default()
                },
                YetiPart,
            ));
            // Left hand/claw
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.65, 0.62, 0.58),
                        custom_size: Some(Vec2::new(14.0, 12.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(-38.0, 24.0, 0.25),
                    ..default()
                },
                YetiPart,
            ));
            // Right arm (reaching out)
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.88, 0.86, 0.84),
                        custom_size: Some(Vec2::new(16.0, 36.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(34.0, 8.0, 0.2)
                        .with_rotation(Quat::from_rotation_z(-0.3)),
                    ..default()
                },
                YetiPart,
            ));
            // Right hand/claw
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.65, 0.62, 0.58),
                        custom_size: Some(Vec2::new(14.0, 12.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(38.0, 24.0, 0.25),
                    ..default()
                },
                YetiPart,
            ));
            // Head
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.9, 0.88, 0.86),
                        custom_size: Some(Vec2::new(36.0, 32.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 38.0, 0.3),
                    ..default()
                },
                YetiPart,
            ));
            // Face (darker area)
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.55, 0.6, 0.7),
                        custom_size: Some(Vec2::new(26.0, 18.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 36.0, 0.35),
                    ..default()
                },
                YetiPart,
            ));
            // Eyes (angry red)
            for x in [-7.0, 7.0] {
                // Eye white
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgb(1.0, 0.95, 0.9),
                            custom_size: Some(Vec2::new(8.0, 6.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(x, 40.0, 0.4),
                        ..default()
                    },
                    YetiPart,
                ));
                // Pupil (angry red glow)
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgb(0.9, 0.15, 0.1),
                            custom_size: Some(Vec2::new(4.0, 5.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(x, 40.0, 0.45),
                        ..default()
                    },
                    YetiPart,
                ));
            }
            // Angry brow ridge
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.45, 0.48, 0.55),
                        custom_size: Some(Vec2::new(28.0, 4.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 44.0, 0.5),
                    ..default()
                },
                YetiPart,
            ));
            // Mouth (snarling)
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.2, 0.1, 0.1),
                        custom_size: Some(Vec2::new(16.0, 6.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 32.0, 0.4),
                    ..default()
                },
                YetiPart,
            ));
            // Teeth
            for x in [-4.0, 0.0, 4.0] {
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgb(0.95, 0.95, 0.85),
                            custom_size: Some(Vec2::new(3.0, 4.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(x, 33.0, 0.45),
                        ..default()
                    },
                    YetiPart,
                ));
            }
            // Horn/spiky fur on head
            for (x, h) in [(-8.0, 10.0), (0.0, 14.0), (8.0, 10.0)] {
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgb(0.85, 0.83, 0.8),
                            custom_size: Some(Vec2::new(8.0, h)),
                            ..default()
                        },
                        transform: Transform::from_xyz(x, 52.0 + h * 0.2, 0.25),
                        ..default()
                    },
                    YetiPart,
                ));
            }
        });

    // Spawn warning text
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "!! YETI INCOMING !!",
                TextStyle {
                    font_size: 32.0,
                    color: Color::srgb(0.95, 0.2, 0.15),
                    ..default()
                },
            )
            .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, WINDOW_HEIGHT / 2.0 - 80.0, 55.0),
            ..default()
        },
        YetiWarningText,
    ));
}

fn update_yeti(
    time: Res<Time>,
    mut game_data: ResMut<GameData>,
    mut yeti_query: Query<(&mut Transform, &mut Yeti), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    mut warning_query: Query<(&mut Text, &mut Transform), (With<YetiWarningText>, Without<Yeti>, Without<Player>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok(player_tf) = player_query.get_single() else { return };
    let player_pos = player_tf.translation;

    for (mut yeti_tf, mut yeti) in &mut yeti_query {
        let dt = time.delta_seconds();

        // Intro: yeti slides down from top of screen to chase position
        if !yeti.intro_done {
            yeti_tf.translation.y -= YETI_INTRO_SPEED * dt;
            // Track player horizontally during intro
            let dx = player_pos.x - yeti_tf.translation.x;
            let track_speed = YETI_CHASE_SPEED_X * dt;
            if dx.abs() > 1.0 {
                yeti_tf.translation.x += dx.signum() * track_speed.min(dx.abs());
            }
            // Menacing sway during intro
            let sway = (time.elapsed_seconds() * 3.0).sin() * 0.08;
            yeti_tf.rotation = Quat::from_rotation_z(sway);

            if yeti_tf.translation.y <= yeti.target_y {
                yeti_tf.translation.y = yeti.target_y;
                yeti.intro_done = true;
            }
            continue;
        }

        // Apply any pending lunge from obstacle hits (instant gap close)
        if game_data.yeti_lunge_pending > 0.0 {
            yeti_tf.translation.y -= game_data.yeti_lunge_pending;
            game_data.yeti_lunge_pending = 0.0;
        }

        let gap = yeti_tf.translation.y - player_pos.y;

        if game_data.stunned_timer > 0.0 {
            // During stun: yeti aggressively closes but stops at MIN_STUN_GAP
            if gap > YETI_MIN_STUN_GAP {
                yeti_tf.translation.y -= YETI_STUN_CLOSE_SPEED * dt;
            }
        } else if gap < YETI_COMFORT_GAP {
            // Not stunned and yeti is too close: slowly retreat
            yeti_tf.translation.y += YETI_RETREAT_SPEED * dt;
            // Don't retreat past comfort gap
            let new_gap = yeti_tf.translation.y - player_pos.y;
            if new_gap > YETI_COMFORT_GAP {
                yeti_tf.translation.y = player_pos.y + YETI_COMFORT_GAP;
            }
        } else {
            // Normal: slow creep toward player
            yeti_tf.translation.y -= YETI_CLOSE_SPEED * dt;
        }

        // Clamp yeti to stay on screen (don't go above top edge)
        let max_y = WINDOW_HEIGHT / 2.0 - 30.0;
        if yeti_tf.translation.y > max_y {
            yeti_tf.translation.y = max_y;
        }

        // Track player horizontally
        let dx = player_pos.x - yeti_tf.translation.x;
        let track_speed = YETI_CHASE_SPEED_X * dt;
        if dx.abs() > 1.0 {
            yeti_tf.translation.x += dx.signum() * track_speed.min(dx.abs());
        }

        // Menacing sway — more aggressive when closer
        let gap_now = (yeti_tf.translation.y - player_pos.y).max(0.0);
        let closeness = 1.0 - (gap_now / YETI_START_DISTANCE).clamp(0.0, 1.0);
        let sway_intensity = 0.06 + closeness * 0.1;
        let sway_speed = 3.0 + closeness * 4.0;
        let sway = (time.elapsed_seconds() * sway_speed).sin() * sway_intensity;
        yeti_tf.rotation = Quat::from_rotation_z(sway);

        // Scale yeti based on proximity — closer = bigger and more menacing
        let scale = 0.7 + closeness * 0.6; // 0.7 when far, 1.3 when right behind
        yeti_tf.scale = Vec3::splat(scale);

        // Check if yeti catches player
        let distance = yeti_tf.translation.truncate().distance(player_pos.truncate());
        if distance < YETI_CATCH_RADIUS {
            let final_score = game_data.score as u32;
            if final_score > game_data.high_score {
                game_data.high_score = final_score;
            }
            next_state.set(GameState::GameOver);
            return;
        }

        // Update warning text — only show when yeti is dangerously close
        for (mut text, mut text_tf) in &mut warning_query {
            if closeness > 0.5 {
                let urgency = (closeness - 0.5) * 2.0; // 0..1 over the close half
                let pulse = ((time.elapsed_seconds() * (6.0 + urgency * 8.0)).sin() * 0.5 + 0.5) * urgency;
                let alpha = 0.3 + pulse * 0.7;
                text.sections[0].style.color = Color::srgba(0.95, 0.15, 0.1, alpha);

                if urgency > 0.7 {
                    text.sections[0].value = "!!! RUN !!!".to_string();
                } else {
                    text.sections[0].value = "!! YETI !!".to_string();
                }

                let shake = if urgency > 0.4 {
                    (time.elapsed_seconds() * 25.0).sin() * urgency * 5.0
                } else {
                    0.0
                };
                text_tf.translation.x = shake;
            } else {
                text.sections[0].style.color = Color::srgba(0.95, 0.15, 0.1, 0.0);
            }
        }
    }
}

// ── Collision Detection ────────────────────────────────────────────────────────

fn check_collisions(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    obstacle_query: Query<(&Transform, &Obstacle)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut game_data: ResMut<GameData>,
    existing_overlay: Query<Entity, With<StunnedOverlay>>,
    sfx: Res<SfxHandles>,
) {
    // Skip collision checks during hit cooldown
    if game_data.hit_cooldown > 0.0 {
        return;
    }

    let Ok(player_tf) = player_query.get_single() else { return };
    let player_pos = player_tf.translation.truncate();

    for (transform, obstacle) in &obstacle_query {
        let obstacle_pos = transform.translation.truncate();
        let distance = player_pos.distance(obstacle_pos);

        if distance < PLAYER_HITBOX_HALF + obstacle.hitbox_radius {
            // Play hit sound
            commands.spawn(AudioBundle {
                source: sfx.hit.clone(),
                settings: PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Despawn,
                    volume: bevy::audio::Volume::new(0.5),
                    ..default()
                },
            });

            if game_data.stunned_timer > 0.0 {
                // Already stunned — second hit: yeti catches you
                game_data.yeti_lunge_pending = 1000.0; // lunge yeti onto player
                let final_score = game_data.score as u32;
                if final_score > game_data.high_score {
                    game_data.high_score = final_score;
                }
                next_state.set(GameState::GameOver);
                return;
            }

            // First hit — stun the player and yeti lunges closer
            game_data.stunned_timer = STUN_DURATION;
            game_data.hit_cooldown = HIT_COOLDOWN;
            game_data.yeti_lunge_pending = YETI_HIT_LUNGE;

            // Red flash overlay
            if existing_overlay.is_empty() {
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgba(0.9, 0.1, 0.05, 0.35),
                            custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
                            ..default()
                        },
                        transform: Transform::from_xyz(0.0, 0.0, 80.0),
                        ..default()
                    },
                    StunnedOverlay,
                ));
                commands.spawn((
                    Text2dBundle {
                        text: Text::from_section(
                            "SLOWED!",
                            TextStyle {
                                font_size: 48.0,
                                color: Color::srgb(1.0, 0.3, 0.2),
                                ..default()
                            },
                        )
                        .with_justify(JustifyText::Center),
                        transform: Transform::from_xyz(0.0, 0.0, 81.0),
                        ..default()
                    },
                    StunnedText,
                ));
            }
            return;
        }
    }
}

// ── Stun System ────────────────────────────────────────────────────────────────

fn update_stun(
    mut commands: Commands,
    time: Res<Time>,
    mut game_data: ResMut<GameData>,
    mut overlay_query: Query<(Entity, &mut Sprite), With<StunnedOverlay>>,
    stun_text_query: Query<Entity, With<StunnedText>>,
) {
    let dt = time.delta_seconds();

    // Tick cooldown
    if game_data.hit_cooldown > 0.0 {
        game_data.hit_cooldown = (game_data.hit_cooldown - dt).max(0.0);
    }

    if game_data.stunned_timer <= 0.0 {
        return;
    }

    game_data.stunned_timer = (game_data.stunned_timer - dt).max(0.0);

    // Pulse the overlay and fade it out as stun wears off
    let stun_pct = game_data.stunned_timer / STUN_DURATION;
    for (_entity, mut sprite) in &mut overlay_query {
        let pulse = (time.elapsed_seconds() * 6.0).sin() * 0.5 + 0.5;
        sprite.color = Color::srgba(0.9, 0.1, 0.05, stun_pct * 0.3 * (0.5 + pulse * 0.5));
    }

    // Stun expired — remove overlay and text, restore speed
    if game_data.stunned_timer <= 0.0 {
        for (entity, _) in &overlay_query {
            commands.entity(entity).despawn();
        }
        for entity in &stun_text_query {
            commands.entity(entity).despawn();
        }
    }
}

// ── Score & Speed ──────────────────────────────────────────────────────────────

fn update_score(time: Res<Time>, mut game_data: ResMut<GameData>, player_query: Query<&Player>) {
    // Don't update score during intro
    if player_query.get_single().map_or(true, |p| !p.intro_done) {
        return;
    }
    // Apply stun slow factor to effective speed
    let speed_mult = if game_data.stunned_timer > 0.0 {
        STUN_SLOW_FACTOR
    } else {
        1.0
    };

    let effective_speed = game_data.scroll_speed * speed_mult;
    game_data.distance += effective_speed * time.delta_seconds();
    game_data.score = game_data.distance / 10.0;

    // Gradually increase base speed (even when stunned, base keeps climbing)
    if game_data.scroll_speed < MAX_SCROLL_SPEED {
        game_data.scroll_speed += SPEED_INCREASE_RATE * time.delta_seconds();
    }
}

fn update_hud(
    game_data: Res<GameData>,
    mut score_query: Query<&mut Text, (With<ScoreText>, Without<SpeedText>)>,
    mut speed_query: Query<&mut Text, (With<SpeedText>, Without<ScoreText>)>,
) {
    for mut text in &mut score_query {
        text.sections[0].value = format!("Score: {}", game_data.score as u32);
    }

    let speed_mult = if game_data.stunned_timer > 0.0 { STUN_SLOW_FACTOR } else { 1.0 };
    let speed_pct = ((game_data.scroll_speed * speed_mult / BASE_SCROLL_SPEED) * 100.0) as u32;
    for mut text in &mut speed_query {
        if game_data.stunned_timer > 0.0 {
            text.sections[0].value = format!("Speed: {}% SLOWED", speed_pct);
            text.sections[0].style.color = Color::srgb(1.0, 0.3, 0.2);
        } else {
            text.sections[0].value = format!("Speed: {}%", speed_pct);
            text.sections[0].style.color = Color::srgb(0.15, 0.15, 0.25);
        }
    }
}

// ── Game Over ──────────────────────────────────────────────────────────────────

fn setup_game_over(mut commands: Commands, game_data: Res<GameData>) {
    // Dark overlay — fully covers the screen
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.0, 0.0, 0.05, 0.85),
                custom_size: Some(Vec2::new(WINDOW_WIDTH * 2.0, WINDOW_HEIGHT * 2.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            ..default()
        },
        GameOverUI,
    ));

    // Panel background
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.08, 0.1, 0.22),
                custom_size: Some(Vec2::new(420.0, 370.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 20.0, 101.0),
            ..default()
        },
        GameOverUI,
    ));

    // Panel border
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.25, 0.4, 0.7),
                custom_size: Some(Vec2::new(426.0, 376.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 20.0, 100.5),
            ..default()
        },
        GameOverUI,
    ));

    // Game Over title
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "WIPEOUT!",
                TextStyle {
                    font_size: 56.0,
                    color: Color::srgb(1.0, 0.35, 0.25),
                    ..default()
                },
            )
            .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, 140.0, 102.0),
            ..default()
        },
        GameOverUI,
    ));

    // Final score
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                format!("Score: {}", game_data.score as u32),
                TextStyle {
                    font_size: 40.0,
                    color: Color::srgb(1.0, 1.0, 1.0),
                    ..default()
                },
            )
            .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, 60.0, 102.0),
            ..default()
        },
        GameOverUI,
        FinalScoreText,
    ));

    // High score
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                format!("Best: {}", game_data.high_score),
                TextStyle {
                    font_size: 30.0,
                    color: Color::srgb(1.0, 0.85, 0.2),
                    ..default()
                },
            )
            .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, 10.0, 102.0),
            ..default()
        },
        GameOverUI,
    ));

    // Distance
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                format!("Distance: {:.0}m", game_data.distance / 10.0),
                TextStyle {
                    font_size: 24.0,
                    color: Color::srgb(0.75, 0.85, 0.95),
                    ..default()
                },
            )
            .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, -30.0, 102.0),
            ..default()
        },
        GameOverUI,
    ));

    // Max speed reached
    let speed_pct = ((game_data.scroll_speed / BASE_SCROLL_SPEED) * 100.0) as u32;
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                format!("Max Speed: {}%", speed_pct),
                TextStyle {
                    font_size: 24.0,
                    color: Color::srgb(0.75, 0.85, 0.95),
                    ..default()
                },
            )
            .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, -65.0, 102.0),
            ..default()
        },
        GameOverUI,
    ));

    // Restart prompt
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "SPACE / R  -  Retry   |   ESC  -  Menu",
                TextStyle {
                    font_size: 22.0,
                    color: Color::srgb(0.7, 0.8, 0.9),
                    ..default()
                },
            )
            .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, -120.0, 102.0),
            ..default()
        },
        GameOverUI,
    ));
}

fn game_over_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::KeyR) {
        next_state.set(GameState::Playing);
    }
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Menu);
    }
}
