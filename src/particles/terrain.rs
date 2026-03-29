use bevy::prelude::*;
use rand::Rng;
use crate::components::{InGame, IcePatch, SideScenery, TerrainMarker};
use crate::constants::*;
use crate::resources::{GameData, SpawnTimers};

/// Spawns terrain details: snow shimmer patches, ice patches, and side scenery.
pub fn spawn_terrain_marks(
    mut commands: Commands,
    time: Res<Time>,
    mut timers: ResMut<SpawnTimers>,
    game_data: Res<GameData>,
) {
    timers.terrain_timer.tick(time.delta());
    if !timers.terrain_timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();
    let spawn_y = -WINDOW_HEIGHT / 2.0 - 20.0;
    let speed_ratio = ((game_data.scroll_speed - BASE_SCROLL_SPEED)
        / (MAX_SCROLL_SPEED - BASE_SCROLL_SPEED))
        .clamp(0.0, 1.0);

    // ── Snow shimmer patches (always) ──
    {
        let x = rng.gen_range(-WINDOW_WIDTH / 2.0 + 30.0..WINDOW_WIDTH / 2.0 - 30.0);
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
                transform: Transform::from_xyz(x, spawn_y, 0.0),
                ..default()
            },
            TerrainMarker { drift },
            InGame,
        ));
    }

    // ── Ice patches (more frequent at higher speeds) ──
    if rng.gen::<f32>() < 0.08 + speed_ratio * 0.15 {
        let x = rng.gen_range(-WINDOW_WIDTH / 2.0 + 60.0..WINDOW_WIDTH / 2.0 - 60.0);
        let w = rng.gen_range(40.0..120.0);
        let h = rng.gen_range(8.0..25.0);
        let shimmer_offset = rng.gen_range(0.0..std::f32::consts::TAU);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.7, 0.85, 0.98, 0.12),
                    custom_size: Some(Vec2::new(w, h)),
                    ..default()
                },
                transform: Transform::from_xyz(x, spawn_y, 0.1),
                ..default()
            },
            IcePatch { shimmer_offset },
            TerrainMarker { drift: 0.0 },
            InGame,
        ));
    }

    // ── Wind-swept snow streaks (at higher speeds) ──
    if speed_ratio > 0.3 && rng.gen::<f32>() < speed_ratio * 0.4 {
        let x = rng.gen_range(-WINDOW_WIDTH / 2.0..WINDOW_WIDTH / 2.0);
        let len = rng.gen_range(30.0..80.0 + speed_ratio * 60.0);
        let alpha = 0.04 + speed_ratio * 0.06;
        let drift: f32 = rng.gen_range(20.0..60.0) * if rng.gen_bool(0.5) { 1.0 } else { -1.0 };

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.9, 0.93, 1.0, alpha),
                    custom_size: Some(Vec2::new(len, 2.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, spawn_y, 0.05)
                    .with_rotation(Quat::from_rotation_z(drift.signum() * 0.15)),
                ..default()
            },
            TerrainMarker { drift },
            InGame,
        ));
    }

    // ── Side scenery (tree lines / snowbanks on edges) ──
    if rng.gen::<f32>() < 0.12 {
        let side = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
        let x = side * (WINDOW_WIDTH / 2.0 - rng.gen_range(10.0..35.0));

        // Snowbank mound
        let w = rng.gen_range(30.0..60.0);
        let h = rng.gen_range(12.0..25.0);
        let shade = rng.gen_range(0.82..0.88);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(shade, shade + 0.03, shade + 0.06),
                    custom_size: Some(Vec2::new(w, h)),
                    ..default()
                },
                transform: Transform::from_xyz(x, spawn_y, 0.2),
                ..default()
            },
            SideScenery,
            TerrainMarker { drift: 0.0 },
            InGame,
        ));

        // Snow cap on mound
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.95, 0.97, 1.0, 0.7),
                    custom_size: Some(Vec2::new(w * 0.7, 4.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, spawn_y + h / 2.0, 0.21),
                ..default()
            },
            SideScenery,
            TerrainMarker { drift: 0.0 },
            InGame,
        ));
    }

    // ── Small edge bushes / rocks ──
    if rng.gen::<f32>() < 0.06 {
        let side = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
        let x = side * (WINDOW_WIDTH / 2.0 - rng.gen_range(5.0..20.0));
        let size = rng.gen_range(6.0..14.0);
        let is_rock = rng.gen_bool(0.5);

        let color = if is_rock {
            Color::srgb(
                rng.gen_range(0.4..0.55),
                rng.gen_range(0.42..0.55),
                rng.gen_range(0.45..0.58),
            )
        } else {
            Color::srgb(
                rng.gen_range(0.15..0.25),
                rng.gen_range(0.4..0.55),
                rng.gen_range(0.15..0.25),
            )
        };

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(size, size * 0.8)),
                    ..default()
                },
                transform: Transform::from_xyz(x, spawn_y, 0.3),
                ..default()
            },
            SideScenery,
            TerrainMarker { drift: 0.0 },
            InGame,
        ));
    }
}

pub fn update_terrain_marks(
    mut commands: Commands,
    time: Res<Time>,
    game_data: Res<GameData>,
    mut query: Query<(Entity, &mut Transform, &TerrainMarker, Option<&IcePatch>, Option<&mut Sprite>)>,
) {
    let dt = time.delta_seconds();
    let elapsed = time.elapsed_seconds();

    for (entity, mut transform, marker, ice, sprite) in &mut query {
        transform.translation.y += game_data.scroll_speed * dt;
        transform.translation.x += marker.drift * dt;

        // Ice patch shimmer effect
        if let (Some(ice), Some(mut spr)) = (ice, sprite) {
            let shimmer = ((elapsed * 3.0 + ice.shimmer_offset).sin() * 0.5 + 0.5) * 0.08 + 0.08;
            spr.color = Color::srgba(0.7, 0.85, 0.98, shimmer);
        }

        if transform.translation.y > WINDOW_HEIGHT / 2.0 + 30.0 {
            if let Some(mut e) = commands.get_entity(entity) { e.despawn(); }
        }
    }
}
