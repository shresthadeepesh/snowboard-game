use bevy::prelude::*;
use crate::components::Player;
use crate::constants::*;
use crate::resources::GameData;
use crate::state::GameState;

pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut game_data: ResMut<GameData>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    let dt = time.delta_seconds();

    // Apply knockback
    if game_data.knockback_timer > 0.0 {
        game_data.knockback_timer -= dt;
    }

    for (mut transform, mut player) in &mut query {
        // Knockback force
        if game_data.knockback_timer > 0.0 {
            transform.translation.x += game_data.knockback_dir.x * KNOCKBACK_FORCE * dt;
        }

        player.sway_timer += dt * IDLE_SWAY_SPEED;

        // Intro slide
        if !player.intro_done {
            transform.translation.y -= PLAYER_INTRO_SPEED * dt;
            if transform.translation.y <= PLAYER_TARGET_Y {
                transform.translation.y = PLAYER_TARGET_Y;
                player.intro_done = true;
            }
        }

        let mut direction: f32 = 0.0;
        if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
            direction -= 1.0;
        }
        if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
            direction += 1.0;
        }

        if direction.abs() > 0.1 {
            transform.translation.x += direction * PLAYER_SPEED * dt;

            let target_lean = -direction * CARVE_LEAN_ANGLE;
            let current = transform.rotation.to_euler(EulerRot::XYZ).2;
            let new_rot = current + (target_lean - current) * 10.0 * dt;
            transform.rotation = Quat::from_rotation_z(new_rot);

            if direction > 0.0 {
                player.sway_timer = std::f32::consts::FRAC_PI_2;
            } else {
                player.sway_timer = -std::f32::consts::FRAC_PI_2;
            }
        } else {
            let sway = player.sway_timer.sin();
            transform.translation.x += sway * IDLE_SWAY_AMOUNT * dt;

            let target_lean = -sway * IDLE_LEAN_AMOUNT;
            let current = transform.rotation.to_euler(EulerRot::XYZ).2;
            let new_rot = current + (target_lean - current) * 6.0 * dt;
            transform.rotation = Quat::from_rotation_z(new_rot);
        }

        let half_width = WINDOW_WIDTH / 2.0 - 30.0;
        transform.translation.x = transform.translation.x.clamp(-half_width, half_width);
    }
}

pub fn pause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
    }
    if keyboard.just_pressed(KeyCode::KeyR) {
        next_state.set(GameState::Playing);
    }
}
