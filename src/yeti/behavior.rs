use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::resources::GameData;
use crate::state::GameState;

pub fn update_yeti(
    time: Res<Time>,
    mut game_data: ResMut<GameData>,
    mut yeti_query: Query<(&mut Transform, &mut Yeti), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    mut warning_query: Query<
        (&mut Text, &mut Transform),
        (With<YetiWarningText>, Without<Yeti>, Without<Player>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok(player_tf) = player_query.get_single() else { return };
    let player_pos = player_tf.translation;

    for (mut yeti_tf, mut yeti) in &mut yeti_query {
        let dt = time.delta_seconds();

        // Intro slide
        if !yeti.intro_done {
            yeti_tf.translation.y -= YETI_INTRO_SPEED * dt;
            let dx = player_pos.x - yeti_tf.translation.x;
            let track_speed = YETI_CHASE_SPEED_X * dt;
            if dx.abs() > 1.0 {
                yeti_tf.translation.x += dx.signum() * track_speed.min(dx.abs());
            }
            let sway = (time.elapsed_seconds() * 3.0).sin() * 0.08;
            yeti_tf.rotation = Quat::from_rotation_z(sway);

            if yeti_tf.translation.y <= yeti.target_y {
                yeti_tf.translation.y = yeti.target_y;
                yeti.intro_done = true;
            }
            continue;
        }

        // Apply pending lunge
        if game_data.yeti_lunge_pending > 0.0 {
            yeti_tf.translation.y -= game_data.yeti_lunge_pending;
            game_data.yeti_lunge_pending = 0.0;
        }

        let gap = yeti_tf.translation.y - player_pos.y;

        if game_data.stunned_timer > 0.0 {
            if gap > YETI_MIN_STUN_GAP {
                yeti_tf.translation.y -= YETI_STUN_CLOSE_SPEED * dt;
            }
        } else if gap < YETI_COMFORT_GAP {
            yeti_tf.translation.y += YETI_RETREAT_SPEED * dt;
            let new_gap = yeti_tf.translation.y - player_pos.y;
            if new_gap > YETI_COMFORT_GAP {
                yeti_tf.translation.y = player_pos.y + YETI_COMFORT_GAP;
            }
        } else {
            yeti_tf.translation.y -= YETI_CLOSE_SPEED * dt;
        }

        // Clamp to screen
        let max_y = WINDOW_HEIGHT / 2.0 - 30.0;
        if yeti_tf.translation.y > max_y {
            yeti_tf.translation.y = max_y;
        }

        // Track player horizontally
        let dx = player_pos.x - yeti_tf.translation.x;
        let track_speed = YETI_CHASE_SPEED_X * dt;
        if dx.abs() > 1.0 {
            yeti_tf.translation.x += dx.signum() * track_speed.min(dx.abs());
        }

        // Menacing sway
        let gap_now = (yeti_tf.translation.y - player_pos.y).max(0.0);
        let closeness = 1.0 - (gap_now / YETI_START_DISTANCE).clamp(0.0, 1.0);
        let sway_intensity = 0.06 + closeness * 0.1;
        let sway_speed = 3.0 + closeness * 4.0;
        let sway = (time.elapsed_seconds() * sway_speed).sin() * sway_intensity;
        yeti_tf.rotation = Quat::from_rotation_z(sway);

        // Scale based on proximity
        let scale = 0.7 + closeness * 0.6;
        yeti_tf.scale = Vec3::splat(scale);

        // Catch check
        let distance = yeti_tf.translation.truncate().distance(player_pos.truncate());
        if distance < YETI_CATCH_RADIUS {
            let final_score = game_data.score as u32;
            if final_score > game_data.high_score {
                game_data.high_score = final_score;
            }
            next_state.set(GameState::GameOver);
            return;
        }

        // Warning text
        for (mut text, mut text_tf) in &mut warning_query {
            if closeness > 0.5 {
                let urgency = (closeness - 0.5) * 2.0;
                let pulse = ((time.elapsed_seconds() * (6.0 + urgency * 8.0)).sin() * 0.5 + 0.5) * urgency;
                let alpha = 0.3 + pulse * 0.7;
                text.sections[0].style.color = Color::srgba(0.95, 0.15, 0.1, alpha);

                text.sections[0].value = if urgency > 0.7 {
                    "!!! RUN !!!".to_string()
                } else {
                    "!! YETI !!".to_string()
                };

                let shake = if urgency > 0.4 {
                    (time.elapsed_seconds() * 25.0).sin() * urgency * 5.0
                } else {
                    0.0
                };
                text_tf.translation.x = shake;
            } else {
                text.sections[0].style.color = Color::srgba(0.95, 0.15, 0.1, 0.0);
            }
        }
    }
}
