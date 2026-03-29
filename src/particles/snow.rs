use bevy::prelude::*;
use rand::Rng;
use crate::components::SnowParticle;
use crate::constants::*;
use crate::resources::{GameData, SpawnTimers};

pub fn spawn_snow(
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
        SnowParticle { lifetime: rng.gen_range(2.0..4.0), drift },
    ));
}

pub fn update_snow(
    mut commands: Commands,
    time: Res<Time>,
    game_data: Res<GameData>,
    mut query: Query<(Entity, &mut Transform, &mut SnowParticle, &mut Sprite)>,
) {
    let speed_mult = game_data.speed_multiplier();
    for (entity, mut transform, mut particle, mut sprite) in &mut query {
        transform.translation.y += game_data.scroll_speed * speed_mult * 0.8 * time.delta_seconds();
        transform.translation.x += particle.drift * time.delta_seconds();
        particle.lifetime -= time.delta_seconds();

        if particle.lifetime < 0.5 {
            let a = (particle.lifetime / 0.5).max(0.0);
            sprite.color = Color::srgba(1.0, 1.0, 1.0, a * 0.6);
        }

        if particle.lifetime <= 0.0 || transform.translation.y > WINDOW_HEIGHT / 2.0 + 20.0 {
            commands.entity(entity).despawn();
        }
    }
}
