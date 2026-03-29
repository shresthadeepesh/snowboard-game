use bevy::prelude::*;

// ── Window ────────────────────────────────────────────────────────────────────

pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 900.0;

// ── Player ────────────────────────────────────────────────────────────────────

pub const PLAYER_SPEED: f32 = 400.0;
pub const PLAYER_HITBOX_HALF: f32 = 18.0;
pub const PLAYER_TARGET_Y: f32 = -WINDOW_HEIGHT / 8.0;
pub const PLAYER_INTRO_SPEED: f32 = 200.0;
pub const CARVE_LEAN_ANGLE: f32 = 0.35;
pub const IDLE_SWAY_SPEED: f32 = 1.8;
pub const IDLE_SWAY_AMOUNT: f32 = 120.0;
pub const IDLE_LEAN_AMOUNT: f32 = 0.18;

// ── Scrolling & Speed ─────────────────────────────────────────────────────────

pub const BASE_SCROLL_SPEED: f32 = 250.0;
pub const MAX_SCROLL_SPEED: f32 = 700.0;
pub const SPEED_INCREASE_RATE: f32 = 8.0;

// ── Spawning ──────────────────────────────────────────────────────────────────

pub const OBSTACLE_SPAWN_INTERVAL: f32 = 0.6;
pub const SNOW_SPAWN_INTERVAL: f32 = 0.02;
pub const TRAIL_SPAWN_INTERVAL: f32 = 0.03;

// ── Yeti ──────────────────────────────────────────────────────────────────────

pub const YETI_SPAWN_DELAY: f32 = 3.0;
pub const YETI_CLOSE_SPEED: f32 = 2.0;
pub const YETI_CHASE_SPEED_X: f32 = 200.0;
pub const YETI_CATCH_RADIUS: f32 = 30.0;
pub const YETI_START_DISTANCE: f32 = 200.0;
pub const YETI_HIT_LUNGE: f32 = 50.0;
pub const YETI_STUN_CLOSE_SPEED: f32 = 15.0;
pub const YETI_MIN_STUN_GAP: f32 = 60.0;
pub const YETI_RETREAT_SPEED: f32 = 20.0;
pub const YETI_COMFORT_GAP: f32 = 140.0;
pub const YETI_INTRO_SPEED: f32 = 250.0;

// ── Stun ──────────────────────────────────────────────────────────────────────

pub const STUN_DURATION: f32 = 3.0;
pub const STUN_SLOW_FACTOR: f32 = 0.4;
pub const HIT_COOLDOWN: f32 = 0.5;

// ── Colors ────────────────────────────────────────────────────────────────────

pub const COLOR_SNOW_BG: Color = Color::srgb(0.92, 0.95, 0.98);
pub const COLOR_PLAYER_BOARD: Color = Color::srgb(0.15, 0.5, 0.85);
pub const COLOR_PLAYER_BODY: Color = Color::srgb(0.9, 0.3, 0.2);
pub const COLOR_PLAYER_HEAD: Color = Color::srgb(0.95, 0.82, 0.7);
pub const COLOR_TREE_TRUNK: Color = Color::srgb(0.45, 0.3, 0.15);
pub const COLOR_TREE_LEAVES: Color = Color::srgb(0.15, 0.55, 0.2);
pub const COLOR_TREE_LEAVES_LIGHT: Color = Color::srgb(0.2, 0.65, 0.3);
pub const COLOR_TREE_SNOW: Color = Color::srgb(0.95, 0.97, 1.0);
pub const COLOR_ROCK_DARK: Color = Color::srgb(0.4, 0.42, 0.45);
pub const COLOR_ROCK_LIGHT: Color = Color::srgb(0.55, 0.57, 0.6);
pub const COLOR_ROCK_HIGHLIGHT: Color = Color::srgb(0.7, 0.72, 0.75);
pub const COLOR_DEER_BODY: Color = Color::srgb(0.6, 0.4, 0.2);
pub const COLOR_DEER_BELLY: Color = Color::srgb(0.85, 0.75, 0.6);
pub const COLOR_DEER_ANTLER: Color = Color::srgb(0.5, 0.35, 0.15);
pub const COLOR_RABBIT_BODY: Color = Color::srgb(0.85, 0.82, 0.78);
pub const COLOR_RABBIT_EAR: Color = Color::srgb(0.95, 0.75, 0.75);
pub const COLOR_TRAIL: Color = Color::srgba(0.7, 0.82, 0.95, 0.5);
pub const COLOR_TITLE: Color = Color::srgb(0.1, 0.3, 0.6);
pub const COLOR_COIN_GOLD: Color = Color::srgb(1.0, 0.84, 0.0);
pub const COLOR_COIN_DARK: Color = Color::srgb(0.85, 0.65, 0.0);
pub const COLOR_SHIELD: Color = Color::srgb(0.3, 0.7, 1.0);
pub const COLOR_MAGNET: Color = Color::srgb(0.9, 0.3, 0.9);
pub const COLOR_SPEED_BOOST: Color = Color::srgb(1.0, 0.5, 0.0);

// ── Screen Shake ──────────────────────────────────────────────────────────────

pub const SHAKE_OBSTACLE_INTENSITY: f32 = 6.0;
pub const SHAKE_OBSTACLE_DURATION: f32 = 0.25;
pub const SHAKE_YETI_INTENSITY: f32 = 14.0;
pub const SHAKE_YETI_DURATION: f32 = 0.5;
pub const SHAKE_DECAY_RATE: f32 = 8.0;
pub const DEATH_FREEZE_DURATION: f32 = 0.7;

// ── Collision Particles ───────────────────────────────────────────────────────

pub const COLLISION_PARTICLE_COUNT: u32 = 12;
pub const COLLISION_PARTICLE_SPEED: f32 = 200.0;
pub const COLLISION_PARTICLE_LIFETIME: f32 = 0.5;

// ── Near-Miss ─────────────────────────────────────────────────────────────────

pub const NEAR_MISS_RADIUS: f32 = 30.0;
pub const NEAR_MISS_BONUS: f32 = 25.0;
pub const NEAR_MISS_TEXT_DURATION: f32 = 0.8;

// ── Floating Text ─────────────────────────────────────────────────────────────

pub const FLOATING_TEXT_RISE_SPEED: f32 = 80.0;

// ── Speed Lines ───────────────────────────────────────────────────────────────

pub const SPEED_LINE_THRESHOLD: f32 = 400.0;
pub const SPEED_LINE_SPAWN_INTERVAL: f32 = 0.02;
pub const SPEED_LINE_LIFETIME: f32 = 0.3;

// ── Knockback ─────────────────────────────────────────────────────────────────

pub const KNOCKBACK_FORCE: f32 = 120.0;
pub const KNOCKBACK_DURATION: f32 = 0.15;

// ── Coins ─────────────────────────────────────────────────────────────────────

pub const COIN_SPAWN_CHANCE: f32 = 0.3;
pub const COIN_COLLECT_RADIUS: f32 = 28.0;
pub const COIN_SCORE_VALUE: f32 = 50.0;
pub const COIN_SPIN_SPEED: f32 = 4.0;

// ── Power-Ups ─────────────────────────────────────────────────────────────────

pub const POWERUP_SPAWN_CHANCE: f32 = 0.08;
pub const POWERUP_COLLECT_RADIUS: f32 = 30.0;
pub const SHIELD_DURATION: f32 = 8.0;
pub const MAGNET_DURATION: f32 = 6.0;
pub const MAGNET_RANGE: f32 = 150.0;
pub const MAGNET_PULL_SPEED: f32 = 400.0;
pub const SPEED_BOOST_DURATION: f32 = 5.0;
pub const SPEED_BOOST_MULTIPLIER: f32 = 1.5;

// ── Combo ─────────────────────────────────────────────────────────────────────

pub const COMBO_TIMEOUT: f32 = 3.0;
pub const COMBO_MULTIPLIER_PER_5: f32 = 0.25;

// ── Moving Obstacles ──────────────────────────────────────────────────────────

pub const ANIMAL_MOVE_SPEED: f32 = 80.0;
pub const ANIMAL_MOVE_CHANCE: f32 = 0.5;

// ── Difficulty Ramping ────────────────────────────────────────────────────────

pub const MIN_OBSTACLE_SPAWN_INTERVAL: f32 = 0.25;
pub const SPAWN_INTERVAL_DECREASE_RATE: f32 = 0.02;

// ── Milestones ────────────────────────────────────────────────────────────────

pub const MILESTONE_DISTANCES: [f32; 4] = [1000.0, 2500.0, 5000.0, 10000.0];
pub const MILESTONE_TEXT_DURATION: f32 = 2.0;

// ── Difficulty Modes ──────────────────────────────────────────────────────────

pub const EASY_SPAWN_MULTIPLIER: f32 = 1.5;
pub const EASY_YETI_SPEED_MULT: f32 = 0.6;
pub const HARD_SPAWN_MULTIPLIER: f32 = 0.7;
pub const HARD_YETI_SPEED_MULT: f32 = 1.4;

// ── Persistence ───────────────────────────────────────────────────────────────

pub const SAVE_FILE_NAME: &str = "snow_rider_save.json";
