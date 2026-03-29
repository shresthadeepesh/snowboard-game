mod snow;
mod trail;
mod terrain;

use bevy::prelude::*;
use crate::components::{BackgroundStrip, ParallaxMountain};
use crate::constants::*;
use crate::resources::GameData;
use crate::state::GameState;

pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                snow::spawn_snow,
                snow::update_snow,
                trail::spawn_trail,
                trail::update_trails,
                terrain::spawn_terrain_marks,
                terrain::update_terrain_marks,
                update_background_tint,
                update_parallax,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

/// Shift background color as speed increases — warmer white → cooler blue/grey.
fn update_background_tint(
    game_data: Res<GameData>,
    mut query: Query<(&mut Sprite, &Transform), With<BackgroundStrip>>,
) {
    if !game_data.is_changed() {
        return;
    }

    let speed_ratio = ((game_data.scroll_speed - BASE_SCROLL_SPEED)
        / (MAX_SCROLL_SPEED - BASE_SCROLL_SPEED))
        .clamp(0.0, 1.0);

    for (mut sprite, tf) in &mut query {
        // Vertical gradient: strips higher on screen are slightly darker
        let y_norm = (tf.translation.y + WINDOW_HEIGHT / 2.0) / WINDOW_HEIGHT;
        let base_shade = 0.88 + y_norm * 0.1;

        // Blend from warm white toward cooler blue-grey as speed increases
        let r = base_shade - speed_ratio * 0.08;
        let g = base_shade + 0.02 - speed_ratio * 0.04;
        let b = base_shade + 0.05 + speed_ratio * 0.03;

        sprite.color = Color::srgb(r, g, b);
    }
}

/// Gently scroll parallax layers upward at fraction of game speed.
fn update_parallax(
    time: Res<Time>,
    game_data: Res<GameData>,
    mut query: Query<(&mut Transform, &ParallaxMountain)>,
) {
    let dt = time.delta_seconds();
    let speed = game_data.scroll_speed * game_data.speed_multiplier();

    for (mut tf, mountain) in &mut query {
        tf.translation.y += speed * mountain.speed_factor * dt;
    }
}
