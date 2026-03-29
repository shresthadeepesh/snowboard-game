use bevy::prelude::*;
use rand::Rng;
use crate::components::{BoostTrail, CarveSpray, InGame, Player, Trail};
use crate::constants::*;
use crate::resources::{GameData, SpawnTimers};

pub fn spawn_trail(
    mut commands: Commands,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut timers: ResMut<SpawnTimers>,
    game_data: Res<GameData>,
    player_query: Query<&Transform, With<Player>>,
) {
    timers.trail_timer.tick(time.delta());
    if !timers.trail_timer.just_finished() {
        return;
    }

    let Ok(player_tf) = player_query.get_single() else { return };
    let px = player_tf.translation.x;
    let py = player_tf.translation.y;
    let mut rng = rand::thread_rng();

    // ── Ski trail ──
    let offset_x = rng.gen_range(-8.0..8.0);
    let is_stunned = game_data.stunned_timer > 0.0;

    let trail_color = if is_stunned {
        Color::srgba(0.9, 0.5, 0.4, 0.4)
    } else {
        COLOR_TRAIL
    };

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: trail_color,
                custom_size: Some(Vec2::new(12.0, 6.0)),
                ..default()
            },
            transform: Transform::from_xyz(px + offset_x, py - 35.0, 4.0),
            ..default()
        },
        Trail { lifetime: 0.8 },
        InGame,
    ));

    // ── Carve spray (when actively turning) ──
    let turning_left = keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA);
    let turning_right = keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD);

    if turning_left || turning_right {
        let spray_dir = if turning_left { 1.0_f32 } else { -1.0 };
        let board_edge_x = px + spray_dir * -20.0; // spray from opposite edge of turn

        for _ in 0..3 {
            let angle = rng.gen_range(-0.4..0.4);
            let speed = rng.gen_range(60.0..140.0);
            let vx = spray_dir * speed * (1.0 + angle);
            let vy = rng.gen_range(20.0..60.0);
            let size = rng.gen_range(2.0..5.0);

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(0.9, 0.93, 1.0, 0.7),
                        custom_size: Some(Vec2::new(size, size)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        board_edge_x + rng.gen_range(-4.0..4.0),
                        py - 25.0 + rng.gen_range(-5.0..5.0),
                        4.5,
                    ),
                    ..default()
                },
                CarveSpray {
                    velocity: Vec2::new(vx, vy),
                    lifetime: rng.gen_range(0.2..0.4),
                },
                InGame,
            ));
        }
    }

    // ── Speed boost fire trail ──
    if game_data.speed_boost_timer > 0.0 {
        for _ in 0..4 {
            let ox = rng.gen_range(-10.0..10.0);
            let size = rng.gen_range(4.0..10.0);
            let r = rng.gen_range(0.9..1.0);
            let g = rng.gen_range(0.3..0.6);

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(r, g, 0.05, 0.7),
                        custom_size: Some(Vec2::new(size, size * 1.5)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        px + ox,
                        py - 30.0 + rng.gen_range(-8.0..0.0),
                        4.2,
                    ),
                    ..default()
                },
                BoostTrail { lifetime: rng.gen_range(0.15..0.35) },
                InGame,
            ));
        }
    }
}

pub fn update_trails(
    mut commands: Commands,
    time: Res<Time>,
    game_data: Res<GameData>,
    mut trail_query: Query<(Entity, &mut Transform, &mut Trail, &mut Sprite), (Without<CarveSpray>, Without<BoostTrail>)>,
    mut spray_query: Query<(Entity, &mut Transform, &mut CarveSpray, &mut Sprite), (Without<Trail>, Without<BoostTrail>)>,
    mut boost_query: Query<(Entity, &mut Transform, &mut BoostTrail, &mut Sprite), (Without<Trail>, Without<CarveSpray>)>,
) {
    let dt = time.delta_seconds();

    // Ski trails
    for (entity, mut tf, mut trail, mut sprite) in &mut trail_query {
        tf.translation.y += game_data.scroll_speed * 0.5 * dt;
        trail.lifetime -= dt;

        let alpha = (trail.lifetime / 0.8).max(0.0) * 0.5;
        sprite.color = Color::srgba(
            sprite.color.to_srgba().red,
            sprite.color.to_srgba().green,
            sprite.color.to_srgba().blue,
            alpha,
        );

        if let Some(ref mut size) = sprite.custom_size {
            size.x += 15.0 * dt;
        }

        if trail.lifetime <= 0.0 {
            if let Some(mut e) = commands.get_entity(entity) { e.despawn(); }
        }
    }

    // Carve spray
    for (entity, mut tf, mut spray, mut sprite) in &mut spray_query {
        tf.translation.x += spray.velocity.x * dt;
        tf.translation.y += spray.velocity.y * dt;
        spray.velocity *= 1.0 - 5.0 * dt;
        spray.lifetime -= dt;

        let alpha = (spray.lifetime / 0.4).max(0.0) * 0.7;
        sprite.color = Color::srgba(0.9, 0.93, 1.0, alpha);

        if spray.lifetime <= 0.0 {
            if let Some(mut e) = commands.get_entity(entity) { e.despawn(); }
        }
    }

    // Boost trail
    for (entity, mut tf, mut bt, mut sprite) in &mut boost_query {
        tf.translation.y += game_data.scroll_speed * 0.3 * dt;
        bt.lifetime -= dt;

        let t = (bt.lifetime / 0.35).max(0.0);
        let c = sprite.color.to_srgba();
        sprite.color = Color::srgba(c.red, c.green * t, 0.05, t * 0.7);

        // Shrink as it fades
        if let Some(ref mut size) = sprite.custom_size {
            size.x *= 1.0 - 3.0 * dt;
            size.y *= 1.0 - 2.0 * dt;
        }

        if bt.lifetime <= 0.0 {
            if let Some(mut e) = commands.get_entity(entity) { e.despawn(); }
        }
    }
}
