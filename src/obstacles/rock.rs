use bevy::prelude::*;
use rand::Rng;
use crate::components::*;
use crate::constants::*;

pub fn spawn_rock(commands: &mut Commands, pos: Vec3, rng: &mut impl Rng) {
    let scale = rng.gen_range(0.7..1.4);
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite { color: Color::NONE, ..default() },
                transform: Transform::from_translation(pos),
                ..default()
            },
            Obstacle { hitbox_radius: 18.0 * scale },
            InGame,
        ))
        .with_children(|parent| {
            parent.spawn((SpriteBundle {
                sprite: Sprite { color: Color::srgba(0.0, 0.0, 0.0, 0.12), custom_size: Some(Vec2::new(38.0 * scale, 12.0 * scale)), ..default() },
                transform: Transform::from_xyz(3.0, -12.0 * scale, -0.1), ..default()
            }, ObstaclePart));
            parent.spawn((SpriteBundle {
                sprite: Sprite { color: COLOR_ROCK_DARK, custom_size: Some(Vec2::new(34.0 * scale, 24.0 * scale)), ..default() },
                transform: Transform::from_xyz(0.0, 0.0, 0.0), ..default()
            }, ObstaclePart));
            parent.spawn((SpriteBundle {
                sprite: Sprite { color: COLOR_ROCK_LIGHT, custom_size: Some(Vec2::new(26.0 * scale, 16.0 * scale)), ..default() },
                transform: Transform::from_xyz(-2.0, 3.0, 0.1), ..default()
            }, ObstaclePart));
            parent.spawn((SpriteBundle {
                sprite: Sprite { color: COLOR_ROCK_HIGHLIGHT, custom_size: Some(Vec2::new(14.0 * scale, 8.0 * scale)), ..default() },
                transform: Transform::from_xyz(-3.0, 6.0 * scale, 0.2), ..default()
            }, ObstaclePart));
            parent.spawn((SpriteBundle {
                sprite: Sprite { color: COLOR_TREE_SNOW, custom_size: Some(Vec2::new(20.0 * scale, 4.0 * scale)), ..default() },
                transform: Transform::from_xyz(0.0, 12.0 * scale, 0.3), ..default()
            }, ObstaclePart));
        });
}

pub fn spawn_big_rock(commands: &mut Commands, pos: Vec3, rng: &mut impl Rng) {
    let scale = rng.gen_range(1.2..1.8);
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite { color: Color::NONE, ..default() },
                transform: Transform::from_translation(pos),
                ..default()
            },
            Obstacle { hitbox_radius: 22.0 * scale },
            InGame,
        ))
        .with_children(|parent| {
            parent.spawn((SpriteBundle {
                sprite: Sprite { color: Color::srgba(0.0, 0.0, 0.0, 0.15), custom_size: Some(Vec2::new(50.0 * scale, 16.0 * scale)), ..default() },
                transform: Transform::from_xyz(4.0, -16.0 * scale, -0.1), ..default()
            }, ObstaclePart));
            parent.spawn((SpriteBundle {
                sprite: Sprite { color: Color::srgb(0.35, 0.37, 0.4), custom_size: Some(Vec2::new(44.0 * scale, 30.0 * scale)), ..default() },
                transform: Transform::from_xyz(0.0, 0.0, 0.0), ..default()
            }, ObstaclePart));
            parent.spawn((SpriteBundle {
                sprite: Sprite { color: COLOR_ROCK_DARK, custom_size: Some(Vec2::new(36.0 * scale, 22.0 * scale)), ..default() },
                transform: Transform::from_xyz(-2.0, 2.0, 0.1), ..default()
            }, ObstaclePart));
            parent.spawn((SpriteBundle {
                sprite: Sprite { color: COLOR_ROCK_HIGHLIGHT, custom_size: Some(Vec2::new(18.0 * scale, 10.0 * scale)), ..default() },
                transform: Transform::from_xyz(-4.0, 8.0 * scale, 0.2), ..default()
            }, ObstaclePart));
            parent.spawn((SpriteBundle {
                sprite: Sprite { color: COLOR_TREE_SNOW, custom_size: Some(Vec2::new(30.0 * scale, 6.0 * scale)), ..default() },
                transform: Transform::from_xyz(0.0, 14.0 * scale, 0.3), ..default()
            }, ObstaclePart));
        });
}
