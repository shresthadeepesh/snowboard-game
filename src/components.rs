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
#[derive(Component)]
pub struct InGame;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct CollisionParticle {
    pub velocity: Vec2,
    pub lifetime: f32,
}

#[derive(Component)]
pub struct FloatingText {
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub rise_speed: f32,
}

#[derive(Component)]
pub struct SpeedLine {
    pub lifetime: f32,
}

#[derive(Component)]
pub struct Coin;

#[derive(Component)]
pub struct PowerUp {
    pub kind: PowerUpKind,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PowerUpKind {
    Shield,
    Magnet,
    SpeedBoost,
}

#[derive(Component)]
pub struct ShieldBubble;

#[derive(Component)]
pub struct MovingObstacle {
    pub direction: f32,
    pub speed: f32,
}

#[derive(Component)]
pub struct HoppingAnimal {
    pub hop_timer: f32,
    pub hop_speed: f32,
}

#[derive(Component)]
pub struct ComboText;

#[derive(Component)]
pub struct CoinCountText;

/// Marks obstacles already checked for near-miss.
#[derive(Component)]
pub struct NearMissChecked;

#[derive(Component)]
pub struct SideScenery;

#[derive(Component)]
pub struct IcePatch {
    pub shimmer_offset: f32,
}

#[derive(Component)]
pub struct CarveSpray {
    pub velocity: Vec2,
    pub lifetime: f32,
}

#[derive(Component)]
pub struct BoostTrail {
    pub lifetime: f32,
}

#[derive(Component)]
pub struct ParallaxMountain {
    pub speed_factor: f32,
}
