use bevy::prelude::*;
use rand::Rng;
use crate::components::TerrainMarker;
use crate::constants::*;
use crate::resources::{GameData, SpawnTimers};

pub fn spawn_terrain_marks(
    mut commands: Commands,
    time: Res<Time>,
    mut timers: ResMut<SpawnTimers>,
) {
    timers.terrain_timer.tick(time.delta());
    if !timers.terrain_timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();
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

pub fn update_terrain_marks(
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
