use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub sway_timer: f32,
    pub intro_done: bool,
}

#[derive(Component)]
pub struct PlayerPart;

#[derive(Component)]
pub struct BackgroundStrip;

#[derive(Component)]
pub struct Obstacle {
    pub hitbox_radius: f32,
}

#[derive(Component)]
pub struct ObstaclePart;

#[derive(Component)]
pub struct SnowParticle {
    pub lifetime: f32,
    pub drift: f32,
}

#[derive(Component)]
pub struct Trail {
    pub lifetime: f32,
}

#[derive(Component)]
pub struct MenuUI;

#[derive(Component)]
pub struct GameOverUI;

#[derive(Component)]
pub struct PausedUI;

#[derive(Component)]
pub struct HudUI;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct SpeedText;

#[derive(Component)]
pub struct FinalScoreText;

#[derive(Component)]
pub struct TerrainMarker {
    pub drift: f32,
}

#[derive(Component)]
pub struct Yeti {
    pub target_y: f32,
    pub intro_done: bool,
}

#[derive(Component)]
pub struct YetiPart;

#[derive(Component)]
pub struct YetiWarningText;

#[derive(Component)]
pub struct StunnedOverlay;

#[derive(Component)]
pub struct StunnedText;

#[derive(Component)]
pub struct BgMusic;

#[derive(Component)]
pub struct SkiSound;

/// Marker for all entities spawned during gameplay.
/// Simplifies cleanup to a single query.
#[derive(Component)]
pub struct InGame;
