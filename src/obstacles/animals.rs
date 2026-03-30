use bevy::prelude::*;
use rand::Rng;
use crate::components::*;
use crate::constants::*;

pub fn spawn_deer(commands: &mut Commands, pos: Vec3, rng: &mut impl Rng) {
    let mut entity = commands.spawn((
        SpriteBundle {
            sprite: Sprite { color: Color::NONE, ..default() },
            transform: Transform::from_translation(pos),
            ..default()
        },
        Obstacle { hitbox_radius: 20.0 },
        InGame,
    ));
    if rng.gen::<f32>() < ANIMAL_MOVE_CHANCE {
        let dir = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
        entity.insert(MovingObstacle { direction: dir, speed: ANIMAL_MOVE_SPEED });
    }
    entity
        .with_children(|parent| {
            parent.spawn((SpriteBundle {
                sprite: Sprite { color: Color::srgba(0.0, 0.0, 0.0, 0.1), custom_size: Some(Vec2::new(40.0, 10.0)), ..default() },
                transform: Transform::from_xyz(0.0, -20.0, -0.1), ..default()
            }, ObstaclePart));
            for x in [-10.0, -4.0, 4.0, 10.0] {
                parent.spawn((SpriteBundle {
                    sprite: Sprite { color: Color::srgb(0.5, 0.33, 0.15), custom_size: Some(Vec2::new(4.0, 18.0)), ..default() },
                    transform: Transform::from_xyz(x, -10.0, 0.0), ..default()
                }, ObstaclePart));
            }
            parent.spawn((SpriteBundle {
                sprite: Sprite { color: COLOR_DEER_BODY, custom_size: Some(Vec2::new(36.0, 20.0)), ..default() },
                transform: Transform::from_xyz(0.0, 4.0, 0.1), ..default()
            }, ObstaclePart));
            parent.spawn((SpriteBundle {
                sprite: Sprite { color: COLOR_DEER_BELLY, custom_size: Some(Vec2::new(28.0, 10.0)), ..default() },
                transform: Transform::from_xyz(0.0, -1.0, 0.15), ..default()
            }, ObstaclePart));
            parent.spawn((SpriteBundle {
                sprite: Sprite { color: COLOR_DEER_BODY, custom_size: Some(Vec2::new(10.0, 16.0)), ..default() },
                transform: Transform::from_xyz(14.0, 16.0, 0.2), ..default()
            }, ObstaclePart));
            parent.spawn((SpriteBundle {
                sprite: Sprite { color: COLOR_DEER_BODY, custom_size: Some(Vec2::new(14.0, 12.0)), ..default() },
                transform: Transform::from_xyz(18.0, 26.0, 0.3), ..default()
            }, ObstaclePart));
            parent.spawn((SpriteBundle {
                sprite: Sprite { color: Color::srgb(0.15, 0.12, 0.1), custom_size: Some(Vec2::new(4.0, 4.0)), ..default() },
                transform: Transform::from_xyz(24.0, 24.0, 0.35), ..default()
            }, ObstaclePart));
            parent.spawn((SpriteBundle {
                sprite: Sprite { color: Color::srgb(0.1, 0.08, 0.06), custom_size: Some(Vec2::new(3.0, 3.0)), ..default() },
                transform: Transform::from_xyz(20.0, 28.0, 0.35), ..default()
            }, ObstaclePart));
            for (dx, dy) in [(16.0, 34.0), (20.0, 38.0), (14.0, 40.0)] {
                parent.spawn((SpriteBundle {
                    sprite: Sprite { color: COLOR_DEER_ANTLER, custom_size: Some(Vec2::new(3.0, 10.0)), ..default() },
                    transform: Transform::from_xyz(dx, dy, 0.4), ..default()
                }, ObstaclePart));
            }
            parent.spawn((SpriteBundle {
                sprite: Sprite { color: COLOR_DEER_BELLY, custom_size: Some(Vec2::new(6.0, 6.0)), ..default() },
                transform: Transform::from_xyz(-18.0, 10.0, 0.15), ..default()
            }, ObstaclePart));
        });
}

pub fn spawn_rabbit(commands: &mut Commands, pos: Vec3, rng: &mut impl Rng) {
    let mut entity = commands.spawn((
        SpriteBundle {
            sprite: Sprite { color: Color::NONE, ..default() },
            transform: Transform::from_translation(pos),
            ..default()
        },
        Obstacle { hitbox_radius: 12.0 },
        InGame,
    ));
    entity.insert(HoppingAnimal {
        hop_timer: rng.gen::<f32>() * std::f32::consts::TAU,
        hop_speed: 5.0 + rng.gen::<f32>() * 2.0,
        hop_height: 8.0 + rng.gen::<f32>() * 4.0,
    });
    if rng.gen::<f32>() < ANIMAL_MOVE_CHANCE {
        let dir = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
        entity.insert(MovingObstacle { direction: dir, speed: ANIMAL_MOVE_SPEED * 1.3 });
    }
    entity
        .with_children(|parent| {
            parent.spawn((SpriteBundle {
                sprite: Sprite { color: Color::srgba(0.0, 0.0, 0.0, 0.08), custom_size: Some(Vec2::new(20.0, 8.0)), ..default() },
                transform: Transform::from_xyz(0.0, -12.0, -0.1), ..default()
            }, ObstaclePart));
            parent.spawn((SpriteBundle {
                sprite: Sprite { color: COLOR_RABBIT_BODY, custom_size: Some(Vec2::new(20.0, 16.0)), ..default() },
                transform: Transform::from_xyz(0.0, -2.0, 0.1), ..default()
            }, ObstaclePart));
            parent.spawn((SpriteBundle {
                sprite: Sprite { color: COLOR_RABBIT_BODY, custom_size: Some(Vec2::new(14.0, 12.0)), ..default() },
                transform: Transform::from_xyz(6.0, 8.0, 0.2), ..default()
            }, ObstaclePart));
            for dx in [3.0, 9.0] {
                parent.spawn((SpriteBundle {
                    sprite: Sprite { color: COLOR_RABBIT_BODY, custom_size: Some(Vec2::new(5.0, 14.0)), ..default() },
                    transform: Transform::from_xyz(dx, 20.0, 0.3), ..default()
                }, ObstaclePart));
                parent.spawn((SpriteBundle {
                    sprite: Sprite { color: COLOR_RABBIT_EAR, custom_size: Some(Vec2::new(3.0, 10.0)), ..default() },
                    transform: Transform::from_xyz(dx, 20.0, 0.35), ..default()
                }, ObstaclePart));
            }
            parent.spawn((SpriteBundle {
                sprite: Sprite { color: Color::srgb(0.15, 0.1, 0.1), custom_size: Some(Vec2::new(3.0, 3.0)), ..default() },
                transform: Transform::from_xyz(10.0, 10.0, 0.35), ..default()
            }, ObstaclePart));
            parent.spawn((SpriteBundle {
                sprite: Sprite { color: Color::srgb(0.9, 0.6, 0.6), custom_size: Some(Vec2::new(2.0, 2.0)), ..default() },
                transform: Transform::from_xyz(12.0, 7.0, 0.35), ..default()
            }, ObstaclePart));
            parent.spawn((SpriteBundle {
                sprite: Sprite { color: Color::WHITE, custom_size: Some(Vec2::new(8.0, 8.0)), ..default() },
                transform: Transform::from_xyz(-10.0, 0.0, 0.15), ..default()
            }, ObstaclePart));
            for x in [-4.0, 4.0] {
                parent.spawn((SpriteBundle {
                    sprite: Sprite { color: Color::srgb(0.78, 0.75, 0.72), custom_size: Some(Vec2::new(6.0, 5.0)), ..default() },
                    transform: Transform::from_xyz(x, -10.0, 0.05), ..default()
                }, ObstaclePart));
            }
        });
}
