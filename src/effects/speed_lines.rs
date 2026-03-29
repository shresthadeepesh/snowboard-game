use bevy::prelude::*;
use rand::Rng;
use crate::components::{InGame, SpeedLine};
use crate::constants::*;
use crate::resources::{GameData, SpawnTimers};

pub fn spawn_speed_lines(
    mut commands: Commands,
    time: Res<Time>,
    mut timers: ResMut<SpawnTimers>,
    game_data: Res<GameData>,
) {
    let effective_speed = game_data.scroll_speed * game_data.speed_multiplier();
    if effective_speed < SPEED_LINE_THRESHOLD {
        return;
    }

    timers.speed_line_timer.tick(time.delta());
    if !timers.speed_line_timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-WINDOW_WIDTH / 2.0..WINDOW_WIDTH / 2.0);
    let y = rng.gen_range(-WINDOW_HEIGHT / 2.0..0.0);
    let height = rng.gen_range(30.0..60.0);
    let speed_ratio = (effective_speed - SPEED_LINE_THRESHOLD) / (MAX_SCROLL_SPEED - SPEED_LINE_THRESHOLD);
    let alpha = 0.1 + speed_ratio.clamp(0.0, 1.0) * 0.2;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(1.0, 1.0, 1.0, alpha),
                custom_size: Some(Vec2::new(2.0, height)),
                ..default()
            },
            transform: Transform::from_xyz(x, y, 25.0),
            ..default()
        },
        SpeedLine { lifetime: SPEED_LINE_LIFETIME },
        InGame,
    ));
}

pub fn update_speed_lines(
    mut commands: Commands,
    time: Res<Time>,
    game_data: Res<GameData>,
    mut query: Query<(Entity, &mut Transform, &mut SpeedLine, &mut Sprite)>,
) {
    let dt = time.delta_seconds();
    let speed = game_data.scroll_speed * game_data.speed_multiplier();

    for (entity, mut tf, mut line, mut sprite) in &mut query {
        tf.translation.y += speed * 2.0 * dt;
        line.lifetime -= dt;

        let alpha = (line.lifetime / SPEED_LINE_LIFETIME).max(0.0) * 0.3;
        sprite.color = Color::srgba(1.0, 1.0, 1.0, alpha);

        if line.lifetime <= 0.0 || tf.translation.y > WINDOW_HEIGHT / 2.0 + 50.0 {
            if let Some(mut e) = commands.get_entity(entity) { e.despawn(); }
        }
    }
}
