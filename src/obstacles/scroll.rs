use bevy::prelude::*;
use crate::components::{HoppingAnimal, MovingObstacle, Obstacle};
use crate::constants::*;
use crate::resources::GameData;

pub fn scroll_obstacles(
    mut commands: Commands,
    time: Res<Time>,
    game_data: Res<GameData>,
    mut query: Query<(Entity, &mut Transform, Option<&mut MovingObstacle>), With<Obstacle>>,
) {
    let speed_mult = game_data.speed_multiplier();
    let dt = time.delta_seconds();

    for (entity, mut transform, moving) in &mut query {
        transform.translation.y += game_data.scroll_speed * speed_mult * dt;

        // Horizontal movement for animals
        if let Some(mut mov) = moving {
            transform.translation.x += mov.direction * mov.speed * dt;
            let bound = WINDOW_WIDTH / 2.0 - 40.0;
            if transform.translation.x.abs() > bound {
                mov.direction = -mov.direction;
                transform.translation.x = transform.translation.x.clamp(-bound, bound);
            }
            // Flip sprite to face movement direction
            // Animals are built facing right (+x), so flip when going left
            transform.scale.x = if mov.direction < 0.0 { -1.0 } else { 1.0 };
        }

        if transform.translation.y > WINDOW_HEIGHT / 2.0 + 100.0 {
            if let Some(e) = commands.get_entity(entity) { e.despawn_recursive(); }
        }
    }
}

pub fn animate_hopping(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut HoppingAnimal, Option<&MovingObstacle>)>,
) {
    let dt = time.delta_seconds();
    for (mut transform, mut hop, moving) in &mut query {
        // Remove previous hop offset
        let old_offset = hop.hop_height * hop.hop_timer.sin().abs();
        transform.translation.y -= old_offset;

        // Only hop when the animal is moving
        if moving.is_none() {
            transform.scale.y = 1.0;
            continue;
        }

        // Advance timer
        hop.hop_timer += dt * hop.hop_speed;

        // Apply new hop offset (abs(sin) gives a bounce shape)
        let new_offset = hop.hop_height * hop.hop_timer.sin().abs();
        transform.translation.y += new_offset;

        // Squash & stretch: compress X and extend Y at peak, reverse at ground
        let phase = hop.hop_timer.sin().abs();
        let squash = 1.0 - phase * 0.15;
        let stretch = 1.0 + phase * 0.2;
        // Preserve flip direction from MovingObstacle
        let flip = transform.scale.x.signum();
        transform.scale.x = flip * squash;
        transform.scale.y = stretch;
    }
}
