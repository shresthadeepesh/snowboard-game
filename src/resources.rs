use bevy::prelude::*;
use crate::constants::*;

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

impl GameData {
    pub fn speed_multiplier(&self) -> f32 {
        if self.stunned_timer > 0.0 {
            STUN_SLOW_FACTOR
        } else {
            1.0
        }
    }

    pub fn reset(&mut self) {
        self.score = 0.0;
        self.scroll_speed = BASE_SCROLL_SPEED;
        self.distance = 0.0;
        self.stunned_timer = 0.0;
        self.hit_cooldown = 0.0;
        self.yeti_lunge_pending = 0.0;
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

#[derive(Resource)]
pub struct SfxHandles {
    pub yeti_roar: Handle<AudioSource>,
    pub hit: Handle<AudioSource>,
    pub ski_loop: Handle<AudioSource>,
}
