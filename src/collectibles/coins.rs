use bevy::prelude::*;
use rand::Rng;
use crate::components::*;
use crate::constants::*;
use crate::resources::{GameData, SpawnTimers, SfxHandles};
use crate::effects::floating_text::spawn_floating_text;

pub fn spawn_coins(
    mut commands: Commands,
    timers: Res<SpawnTimers>,
    player_query: Query<&Player>,
) {
    if player_query.get_single().map_or(true, |p| !p.intro_done) {
        return;
    }

    // Piggyback on obstacle spawn timer — only spawn if timer just fired
    if !timers.obstacle_timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();
    if rng.gen::<f32>() > COIN_SPAWN_CHANCE {
        return;
    }

    let x = rng.gen_range(-WINDOW_WIDTH / 2.0 + 40.0..WINDOW_WIDTH / 2.0 - 40.0);
    let y = -WINDOW_HEIGHT / 2.0 - 80.0;

    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: COLOR_COIN_GOLD,
                    custom_size: Some(Vec2::new(16.0, 16.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 8.0),
                ..default()
            },
            Coin,
            InGame,
        ))
        .with_children(|parent| {
            // Inner darker square for depth
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: COLOR_COIN_DARK,
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 0.1),
                ..default()
            });
        });
}

pub fn update_coins(
    mut commands: Commands,
    time: Res<Time>,
    mut game_data: ResMut<GameData>,
    sfx: Res<SfxHandles>,
    player_query: Query<&Transform, With<Player>>,
    mut coin_query: Query<(Entity, &mut Transform), (With<Coin>, Without<Player>)>,
) {
    let Ok(player_tf) = player_query.get_single() else { return };
    let player_pos = player_tf.translation.truncate();
    let dt = time.delta_seconds();
    let speed_mult = game_data.speed_multiplier();
    let elapsed = time.elapsed_seconds();

    for (entity, mut tf) in &mut coin_query {
        // Scroll upward
        tf.translation.y += game_data.scroll_speed * speed_mult * dt;

        // Spin effect (scale x oscillation)
        let spin = (elapsed * COIN_SPIN_SPEED + tf.translation.x).cos().abs();
        tf.scale.x = 0.6 + spin * 0.4;

        // Magnet pull
        if game_data.magnet_timer > 0.0 {
            let coin_pos = tf.translation.truncate();
            let diff = player_pos - coin_pos;
            let dist = diff.length();
            if dist < MAGNET_RANGE && dist > 1.0 {
                let pull = diff.normalize() * MAGNET_PULL_SPEED * dt;
                tf.translation.x += pull.x;
                tf.translation.y += pull.y;
            }
        }

        // Collection check
        let dist = player_pos.distance(tf.translation.truncate());
        if dist < COIN_COLLECT_RADIUS {
            game_data.coins_collected += 1;
            game_data.score += COIN_SCORE_VALUE * game_data.score_multiplier;

            spawn_floating_text(
                &mut commands,
                &format!("+{}", (COIN_SCORE_VALUE * game_data.score_multiplier) as u32),
                tf.translation.truncate() + Vec2::new(0.0, 15.0),
                COLOR_COIN_GOLD,
                20.0,
                0.6,
                FLOATING_TEXT_RISE_SPEED,
            );

            // Play coin sound
            commands.spawn(AudioBundle {
                source: sfx.coin.clone(),
                settings: PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Despawn,
                    volume: bevy::audio::Volume::new(0.4),
                    ..default()
                },
            });

            if let Some(e) = commands.get_entity(entity) { e.despawn_recursive(); }
            continue;
        }

        // Despawn if off-screen
        if tf.translation.y > WINDOW_HEIGHT / 2.0 + 100.0 {
            if let Some(e) = commands.get_entity(entity) { e.despawn_recursive(); }
        }
    }
}
