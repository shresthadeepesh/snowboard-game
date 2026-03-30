use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::resources::{GameData, Settings, SfxHandles};
use super::floating_text::spawn_floating_text;

/// Check obstacles that have scrolled past the player for near-misses and dodges.
pub fn check_near_misses(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    sfx: Res<SfxHandles>,
    settings: Res<Settings>,
    mut obstacle_query: Query<
        (Entity, &Transform, &Obstacle),
        (Without<Player>, Without<NearMissChecked>),
    >,
    mut game_data: ResMut<GameData>,
) {
    let Ok(player_tf) = player_query.get_single() else { return };
    let player_pos = player_tf.translation.truncate();

    for (entity, tf, obstacle) in &mut obstacle_query {
        let obs_pos = tf.translation.truncate();

        // Only check obstacles that have scrolled past the player
        if tf.translation.y <= player_pos.y {
            continue;
        }
        // Only check if close enough vertically (just passed)
        if tf.translation.y > player_pos.y + 60.0 {
            continue;
        }

        let dist = player_pos.distance(obs_pos);
        let hit_dist = PLAYER_HITBOX_HALF + obstacle.hitbox_radius;

        commands.entity(entity).insert(NearMissChecked);

        // Only count obstacles that were actually in the player's path (within reasonable range)
        if dist > hit_dist + NEAR_MISS_RADIUS * 4.0 {
            continue;
        }

        // Count as dodged
        game_data.obstacles_dodged += 1;
        game_data.combo_count += 1;
        game_data.combo_timer = COMBO_TIMEOUT;
        if game_data.combo_count > game_data.max_combo {
            game_data.max_combo = game_data.combo_count;
        }
        game_data.score_multiplier =
            1.0 + (game_data.combo_count / 5) as f32 * COMBO_MULTIPLIER_PER_5;

        // Near-miss bonus (close dodge)
        if dist < hit_dist + NEAR_MISS_RADIUS {
            game_data.near_misses += 1;
            game_data.score += NEAR_MISS_BONUS * game_data.score_multiplier;
            spawn_floating_text(
                &mut commands,
                "NICE!",
                Vec2::new(obs_pos.x, obs_pos.y + 20.0),
                Color::srgb(0.2, 1.0, 0.4),
                24.0,
                NEAR_MISS_TEXT_DURATION,
                FLOATING_TEXT_RISE_SPEED,
            );
            commands.spawn(AudioBundle {
                source: sfx.near_miss.clone(),
                settings: PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Despawn,
                    volume: bevy::audio::Volume::new(0.3 * settings.effective_sfx()),
                    ..default()
                },
            });
        }
    }
}

/// Tick combo timer, reset combo on timeout.
pub fn update_combo(time: Res<Time>, mut game_data: ResMut<GameData>) {
    if game_data.combo_count == 0 {
        return;
    }
    game_data.combo_timer -= time.delta_seconds();
    if game_data.combo_timer <= 0.0 {
        game_data.combo_count = 0;
        game_data.score_multiplier = 1.0;
    }
}
