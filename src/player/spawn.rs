use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;

pub fn spawn_background(commands: &mut Commands) {
    for i in 0..20 {
        let y = -WINDOW_HEIGHT / 2.0 + i as f32 * 50.0;
        let shade = 0.88 + (i as f32 * 0.005);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(shade, shade + 0.02, shade + 0.05),
                    custom_size: Some(Vec2::new(WINDOW_WIDTH, 50.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, y, -0.5),
                ..default()
            },
            BackgroundStrip,
            InGame,
        ));
    }
}

pub fn spawn_player(commands: &mut Commands) {
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::NONE,
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, WINDOW_HEIGHT / 2.0 + 60.0, 10.0),
                ..default()
            },
            Player { sway_timer: 0.0, intro_done: false },
            InGame,
        ))
        .with_children(|parent| {
            // Shadow
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(0.0, 0.0, 0.0, 0.15),
                        custom_size: Some(Vec2::new(48.0, 12.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, -28.0, -0.1),
                    ..default()
                },
                PlayerPart,
            ));
            // Snowboard
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_PLAYER_BOARD,
                        custom_size: Some(Vec2::new(50.0, 8.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, -22.0, 0.0),
                    ..default()
                },
                PlayerPart,
            ));
            // Board edge highlights
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.1, 0.35, 0.7),
                        custom_size: Some(Vec2::new(52.0, 2.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, -18.0, 0.01),
                    ..default()
                },
                PlayerPart,
            ));
            // Legs
            for x in [-6.0, 6.0] {
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgb(0.2, 0.25, 0.35),
                            custom_size: Some(Vec2::new(7.0, 14.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(x, -10.0, 0.1),
                        ..default()
                    },
                    PlayerPart,
                ));
            }
            // Body / jacket
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_PLAYER_BODY,
                        custom_size: Some(Vec2::new(22.0, 26.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 8.0, 0.2),
                    ..default()
                },
                PlayerPart,
            ));
            // Jacket stripe
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.95, 0.85, 0.2),
                        custom_size: Some(Vec2::new(22.0, 4.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 4.0, 0.25),
                    ..default()
                },
                PlayerPart,
            ));
            // Arms
            for x in [-14.0, 14.0] {
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: COLOR_PLAYER_BODY,
                            custom_size: Some(Vec2::new(8.0, 6.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(x, 12.0, 0.2),
                        ..default()
                    },
                    PlayerPart,
                ));
            }
            // Head
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: COLOR_PLAYER_HEAD,
                        custom_size: Some(Vec2::new(16.0, 16.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 28.0, 0.3),
                    ..default()
                },
                PlayerPart,
            ));
            // Goggles
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.1, 0.1, 0.15),
                        custom_size: Some(Vec2::new(14.0, 5.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 30.0, 0.35),
                    ..default()
                },
                PlayerPart,
            ));
            // Goggle lens
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.3, 0.7, 0.95),
                        custom_size: Some(Vec2::new(10.0, 3.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 30.0, 0.36),
                    ..default()
                },
                PlayerPart,
            ));
            // Beanie hat
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.8, 0.2, 0.15),
                        custom_size: Some(Vec2::new(18.0, 10.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 38.0, 0.3),
                    ..default()
                },
                PlayerPart,
            ));
            // Hat pom-pom
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::WHITE,
                        custom_size: Some(Vec2::new(8.0, 8.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 44.0, 0.35),
                    ..default()
                },
                PlayerPart,
            ));
        });
}
