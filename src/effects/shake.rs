use bevy::prelude::*;
use crate::components::MainCamera;
use crate::constants::*;
use crate::resources::GameData;

pub fn apply_screen_shake(
    time: Res<Time>,
    mut game_data: ResMut<GameData>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    let dt = time.delta_seconds();

    if game_data.shake_timer <= 0.0 {
        // Reset camera to center
        for mut tf in &mut camera_query {
            tf.translation.x = 0.0;
            tf.translation.y = 0.0;
        }
        return;
    }

    game_data.shake_timer -= dt;
    game_data.shake_intensity *= 1.0 - SHAKE_DECAY_RATE * dt;

    let elapsed = time.elapsed_seconds();
    let offset_x = (elapsed * 73.0).sin() * game_data.shake_intensity;
    let offset_y = (elapsed * 97.0).cos() * game_data.shake_intensity;

    for mut tf in &mut camera_query {
        tf.translation.x = offset_x;
        tf.translation.y = offset_y;
    }

    if game_data.shake_timer <= 0.0 {
        game_data.shake_intensity = 0.0;
    }
}
