use bevy::prelude::*;
use rand::Rng;
use crate::components::{Player, Trail};
use crate::constants::*;
use crate::resources::{GameData, SpawnTimers};

pub fn spawn_trail(
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

pub fn update_trails(
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

        if let Some(ref mut size) = sprite.custom_size {
            size.x += 15.0 * time.delta_seconds();
        }

        if trail.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
