use bevy::prelude::*;
use rand::Rng;
use crate::components::*;
use crate::constants::*;
use crate::resources::{GameData, SpawnTimers, SfxHandles};
use crate::effects::floating_text::spawn_floating_text;

pub fn spawn_powerups(
    mut commands: Commands,
    timers: Res<SpawnTimers>,
    player_query: Query<&Player>,
) {
    if player_query.get_single().map_or(true, |p| !p.intro_done) {
        return;
    }
    if !timers.obstacle_timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();
    if rng.gen::<f32>() > POWERUP_SPAWN_CHANCE {
        return;
    }

    let x = rng.gen_range(-WINDOW_WIDTH / 2.0 + 60.0..WINDOW_WIDTH / 2.0 - 60.0);
    let y = -WINDOW_HEIGHT / 2.0 - 80.0;
    let kind = match rng.gen_range(0..3) {
        0 => PowerUpKind::Shield,
        1 => PowerUpKind::Magnet,
        _ => PowerUpKind::SpeedBoost,
    };

    let (color, label) = match kind {
        PowerUpKind::Shield => (COLOR_SHIELD, "S"),
        PowerUpKind::Magnet => (COLOR_MAGNET, "M"),
        PowerUpKind::SpeedBoost => (COLOR_SPEED_BOOST, "B"),
    };

    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(24.0, 24.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 8.0),
                ..default()
            },
            PowerUp { kind },
            InGame,
        ))
        .with_children(|parent| {
            parent.spawn(Text2dBundle {
                text: Text::from_section(
                    label,
                    TextStyle {
                        font_size: 16.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_justify(JustifyText::Center),
                transform: Transform::from_xyz(0.0, 0.0, 0.1),
                ..default()
            });
        });
}

pub fn collect_powerups(
    mut commands: Commands,
    mut game_data: ResMut<GameData>,
    sfx: Res<SfxHandles>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    powerup_query: Query<(Entity, &Transform, &PowerUp), Without<Player>>,
) {
    let Ok((player_entity, player_tf)) = player_query.get_single() else { return };
    let player_pos = player_tf.translation.truncate();

    for (entity, tf, powerup) in &powerup_query {
        let dist = player_pos.distance(tf.translation.truncate());
        if dist < POWERUP_COLLECT_RADIUS {
            match powerup.kind {
                PowerUpKind::Shield => {
                    game_data.shield_timer = SHIELD_DURATION;
                    // Spawn rounded shield bubble on player
                    commands.entity(player_entity).with_children(|parent| {
                        let radius = 42.0;
                        let segments = 24;
                        // Ring of small blocks forming a circle outline
                        for i in 0..segments {
                            let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;
                            let x = angle.cos() * radius;
                            let y = angle.sin() * radius + 5.0;
                            parent.spawn((
                                SpriteBundle {
                                    sprite: Sprite {
                                        color: Color::srgba(0.3, 0.75, 1.0, 0.5),
                                        custom_size: Some(Vec2::new(12.0, 12.0)),
                                        ..default()
                                    },
                                    transform: Transform::from_xyz(x, y, 0.5),
                                    ..default()
                                },
                                ShieldBubble,
                            ));
                        }
                        // Inner glow layers (concentric, fading)
                        for (r, alpha) in [(36.0, 0.08), (26.0, 0.06), (16.0, 0.04)] {
                            let inner_segs = 16;
                            for i in 0..inner_segs {
                                let angle = (i as f32 / inner_segs as f32) * std::f32::consts::TAU;
                                let x = angle.cos() * r;
                                let y = angle.sin() * r + 5.0;
                                parent.spawn((
                                    SpriteBundle {
                                        sprite: Sprite {
                                            color: Color::srgba(0.4, 0.8, 1.0, alpha),
                                            custom_size: Some(Vec2::new(14.0, 14.0)),
                                            ..default()
                                        },
                                        transform: Transform::from_xyz(x, y, 0.48),
                                        ..default()
                                    },
                                    ShieldBubble,
                                ));
                            }
                        }
                    });
                    spawn_floating_text(
                        &mut commands,
                        "SHIELD!",
                        player_pos + Vec2::new(0.0, 40.0),
                        COLOR_SHIELD,
                        22.0,
                        1.0,
                        FLOATING_TEXT_RISE_SPEED,
                    );
                }
                PowerUpKind::Magnet => {
                    game_data.magnet_timer = MAGNET_DURATION;
                    spawn_floating_text(
                        &mut commands,
                        "MAGNET!",
                        player_pos + Vec2::new(0.0, 40.0),
                        COLOR_MAGNET,
                        22.0,
                        1.0,
                        FLOATING_TEXT_RISE_SPEED,
                    );
                }
                PowerUpKind::SpeedBoost => {
                    game_data.speed_boost_timer = SPEED_BOOST_DURATION;
                    spawn_floating_text(
                        &mut commands,
                        "BOOST!",
                        player_pos + Vec2::new(0.0, 40.0),
                        COLOR_SPEED_BOOST,
                        22.0,
                        1.0,
                        FLOATING_TEXT_RISE_SPEED,
                    );
                }
            }

            commands.spawn(AudioBundle {
                source: sfx.powerup.clone(),
                settings: PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Despawn,
                    volume: bevy::audio::Volume::new(0.5),
                    ..default()
                },
            });

            if let Some(e) = commands.get_entity(entity) { e.despawn_recursive(); }
        }
    }
}

pub fn update_powerup_timers(
    mut commands: Commands,
    time: Res<Time>,
    mut game_data: ResMut<GameData>,
    shield_query: Query<Entity, With<ShieldBubble>>,
    mut powerup_scroll: Query<(Entity, &mut Transform), (With<PowerUp>, Without<Player>)>,
) {
    let dt = time.delta_seconds();
    let speed_mult = game_data.speed_multiplier();

    // Scroll powerups
    for (entity, mut tf) in &mut powerup_scroll {
        tf.translation.y += game_data.scroll_speed * speed_mult * dt;
        if tf.translation.y > WINDOW_HEIGHT / 2.0 + 100.0 {
            if let Some(e) = commands.get_entity(entity) { e.despawn_recursive(); }
        }
    }

    // Tick timers
    if game_data.shield_timer > 0.0 {
        game_data.shield_timer = (game_data.shield_timer - dt).max(0.0);
        if game_data.shield_timer <= 0.0 {
            for entity in &shield_query {
                if let Some(mut e) = commands.get_entity(entity) { e.despawn(); }
            }
        }
    }
    if game_data.magnet_timer > 0.0 {
        game_data.magnet_timer = (game_data.magnet_timer - dt).max(0.0);
    }
    if game_data.speed_boost_timer > 0.0 {
        game_data.speed_boost_timer = (game_data.speed_boost_timer - dt).max(0.0);
    }
}
