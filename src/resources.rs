use bevy::prelude::*;
use crate::constants::*;

#[derive(Clone, Copy, Default, PartialEq)]
pub enum DifficultyMode {
    Easy,
    #[default]
    Normal,
    Hard,
}

impl DifficultyMode {
    pub fn spawn_multiplier(self) -> f32 {
        match self {
            DifficultyMode::Easy => EASY_SPAWN_MULTIPLIER,
            DifficultyMode::Normal => 1.0,
            DifficultyMode::Hard => HARD_SPAWN_MULTIPLIER,
        }
    }

    pub fn yeti_speed_multiplier(self) -> f32 {
        match self {
            DifficultyMode::Easy => EASY_YETI_SPEED_MULT,
            DifficultyMode::Normal => 1.0,
            DifficultyMode::Hard => HARD_YETI_SPEED_MULT,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            DifficultyMode::Easy => "Easy",
            DifficultyMode::Normal => "Normal",
            DifficultyMode::Hard => "Hard",
        }
    }
}

#[derive(Resource)]
pub struct GameData {
    pub score: f32,
    pub scroll_speed: f32,
    pub distance: f32,
    pub high_score: u32,
    pub stunned_timer: f32,
    pub hit_cooldown: f32,
    pub yeti_lunge_pending: f32,
    pub is_paused_resume: bool,

    // Combo
    pub combo_count: u32,
    pub combo_timer: f32,
    pub score_multiplier: f32,

    // Coins
    pub coins_collected: u32,

    // Power-ups
    pub shield_timer: f32,
    pub magnet_timer: f32,
    pub speed_boost_timer: f32,

    // Knockback
    pub knockback_dir: Vec2,
    pub knockback_timer: f32,

    // Screen shake
    pub shake_intensity: f32,
    pub shake_timer: f32,

    // Death delay
    pub death_timer: f32,

    // Run statistics
    pub obstacles_dodged: u32,
    pub near_misses: u32,
    pub max_combo: u32,

    // Milestones
    pub next_milestone_index: usize,

    // Difficulty (persists across reset)
    pub difficulty: DifficultyMode,
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
            combo_count: 0,
            combo_timer: 0.0,
            score_multiplier: 1.0,
            coins_collected: 0,
            shield_timer: 0.0,
            magnet_timer: 0.0,
            speed_boost_timer: 0.0,
            knockback_dir: Vec2::ZERO,
            knockback_timer: 0.0,
            shake_intensity: 0.0,
            shake_timer: 0.0,
            death_timer: 0.0,
            obstacles_dodged: 0,
            near_misses: 0,
            max_combo: 0,
            next_milestone_index: 0,
            difficulty: DifficultyMode::Normal,
        }
    }
}

impl GameData {
    pub fn speed_multiplier(&self) -> f32 {
        let base = if self.stunned_timer > 0.0 {
            STUN_SLOW_FACTOR
        } else {
            1.0
        };
        if self.speed_boost_timer > 0.0 {
            base * SPEED_BOOST_MULTIPLIER
        } else {
            base
        }
    }

    pub fn reset(&mut self) {
        let difficulty = self.difficulty;
        let high_score = self.high_score;
        *self = Self {
            difficulty,
            high_score,
            ..Default::default()
        };
    }
}

#[derive(Resource)]
pub struct SpawnTimers {
    pub obstacle_timer: Timer,
    pub snow_timer: Timer,
    pub trail_timer: Timer,
    pub terrain_timer: Timer,
    pub yeti_timer: Timer,
    pub yeti_spawned: bool,
    pub speed_line_timer: Timer,
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
            speed_line_timer: Timer::from_seconds(SPEED_LINE_SPAWN_INTERVAL, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
pub struct SfxHandles {
    pub yeti_roar: Handle<AudioSource>,
    pub hit: Handle<AudioSource>,
    pub ski_loop: Handle<AudioSource>,
    pub coin: Handle<AudioSource>,
    pub powerup: Handle<AudioSource>,
    pub near_miss: Handle<AudioSource>,
    pub game_over: Handle<AudioSource>,
}

#[derive(Resource, Default)]
pub struct PersistentData {
    pub high_score: u32,
    pub loaded: bool,
}
