use bevy::prelude::*;
use crate::components::Obstacle;
use crate::constants::*;
use crate::resources::GameData;

pub fn scroll_obstacles(
    mut commands: Commands,
    time: Res<Time>,
    game_data: Res<GameData>,
    mut query: Query<(Entity, &mut Transform), With<Obstacle>>,
) {
    let speed_mult = game_data.speed_multiplier();
    for (entity, mut transform) in &mut query {
        transform.translation.y += game_data.scroll_speed * speed_mult * time.delta_seconds();
        if transform.translation.y > WINDOW_HEIGHT / 2.0 + 100.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
