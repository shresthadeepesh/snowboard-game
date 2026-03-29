use bevy::prelude::*;
use crate::components::{MovingObstacle, Obstacle};
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
