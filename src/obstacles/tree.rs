use bevy::prelude::*;
use rand::Rng;
use crate::components::*;
use crate::constants::*;

pub fn spawn_tree(commands: &mut Commands, pos: Vec3, rng: &mut impl Rng) {
    let scale = rng.gen_range(0.8..1.3);
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite { color: Color::NONE, ..default() },
                transform: Transform::from_translation(pos),
                ..default()
            },
            Obstacle { hitbox_radius: 15.0 * scale },
            InGame,
        ))
        .with_children(|parent| {
            parent.spawn((SpriteBundle {
                sprite: Sprite { color: Color::srgba(0.0, 0.0, 0.0, 0.12), custom_size: Some(Vec2::new(40.0 * scale, 14.0 * scale)), ..default() },
                transform: Transform::from_xyz(4.0, -20.0 * scale, -0.1), ..default()
            }, ObstaclePart));
            parent.spawn((SpriteBundle {
                sprite: Sprite { color: COLOR_TREE_TRUNK, custom_size: Some(Vec2::new(10.0 * scale, 28.0 * scale)), ..default() },
                transform: Transform::from_xyz(0.0, 0.0, 0.0), ..default()
            }, ObstaclePart));
            parent.spawn((SpriteBundle {
                sprite: Sprite { color: COLOR_TREE_LEAVES, custom_size: Some(Vec2::new(40.0 * scale, 22.0 * scale)), ..default() },
                transform: Transform::from_xyz(0.0, 14.0 * scale, 0.1), ..default()
            }, ObstaclePart));
            parent.spawn((SpriteBundle {
                sprite: Sprite { color: COLOR_TREE_LEAVES_LIGHT, custom_size: Some(Vec2::new(30.0 * scale, 20.0 * scale)), ..default() },
                transform: Transform::from_xyz(0.0, 28.0 * scale, 0.2), ..default()
            }, ObstaclePart));
            parent.spawn((SpriteBundle {
                sprite: Sprite { color: COLOR_TREE_LEAVES, custom_size: Some(Vec2::new(20.0 * scale, 18.0 * scale)), ..default() },
                transform: Transform::from_xyz(0.0, 42.0 * scale, 0.3), ..default()
            }, ObstaclePart));
            parent.spawn((SpriteBundle {
                sprite: Sprite { color: COLOR_TREE_SNOW, custom_size: Some(Vec2::new(22.0 * scale, 6.0 * scale)), ..default() },
                transform: Transform::from_xyz(0.0, 48.0 * scale, 0.4), ..default()
            }, ObstaclePart));
            parent.spawn((SpriteBundle {
                sprite: Sprite { color: COLOR_TREE_SNOW, custom_size: Some(Vec2::new(28.0 * scale, 4.0 * scale)), ..default() },
                transform: Transform::from_xyz(2.0, 24.0 * scale, 0.35), ..default()
            }, ObstaclePart));
        });
}
