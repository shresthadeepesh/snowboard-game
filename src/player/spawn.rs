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
            spawn_shadow(parent);
            spawn_snowboard(parent);
            spawn_body(parent);
        });
}

fn spawn_shadow(parent: &mut ChildBuilder) {
    // Main shadow under board
    parent.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.0, 0.0, 0.0, 0.12),
                custom_size: Some(Vec2::new(56.0, 10.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -30.0, -0.1),
            ..default()
        },
        PlayerPart,
    ));
}

fn spawn_snowboard(parent: &mut ChildBuilder) {
    let board_y = -23.0;
    let board_color = COLOR_PLAYER_BOARD;
    let dark_blue = Color::srgb(0.08, 0.25, 0.55);
    let light_blue = Color::srgb(0.3, 0.65, 1.0);
    let accent = Color::srgb(0.95, 0.3, 0.15);

    // Board underside/edge (slightly larger, darker — gives depth)
    parent.spawn((
        SpriteBundle {
            sprite: Sprite { color: dark_blue, custom_size: Some(Vec2::new(48.0, 9.0)), ..default() },
            transform: Transform::from_xyz(0.0, board_y - 1.0, -0.01), ..default()
        },
        PlayerPart,
    ));

    // Main deck
    parent.spawn((
        SpriteBundle {
            sprite: Sprite { color: board_color, custom_size: Some(Vec2::new(46.0, 7.0)), ..default() },
            transform: Transform::from_xyz(0.0, board_y, 0.0), ..default()
        },
        PlayerPart,
    ));

    // Top surface sheen
    parent.spawn((
        SpriteBundle {
            sprite: Sprite { color: light_blue, custom_size: Some(Vec2::new(40.0, 2.5)), ..default() },
            transform: Transform::from_xyz(0.0, board_y + 1.5, 0.01), ..default()
        },
        PlayerPart,
    ));

    // Nose (front tip) — curved upward
    parent.spawn((
        SpriteBundle {
            sprite: Sprite { color: board_color, custom_size: Some(Vec2::new(10.0, 6.5)), ..default() },
            transform: Transform::from_xyz(25.0, board_y + 1.5, 0.0)
                .with_rotation(Quat::from_rotation_z(0.3)),
            ..default()
        },
        PlayerPart,
    ));
    // Nose accent
    parent.spawn((
        SpriteBundle {
            sprite: Sprite { color: accent, custom_size: Some(Vec2::new(5.0, 5.5)), ..default() },
            transform: Transform::from_xyz(29.0, board_y + 3.0, 0.02)
                .with_rotation(Quat::from_rotation_z(0.3)),
            ..default()
        },
        PlayerPart,
    ));

    // Tail (back tip) — curved upward
    parent.spawn((
        SpriteBundle {
            sprite: Sprite { color: board_color, custom_size: Some(Vec2::new(10.0, 6.5)), ..default() },
            transform: Transform::from_xyz(-25.0, board_y + 1.5, 0.0)
                .with_rotation(Quat::from_rotation_z(-0.3)),
            ..default()
        },
        PlayerPart,
    ));
    // Tail accent
    parent.spawn((
        SpriteBundle {
            sprite: Sprite { color: accent, custom_size: Some(Vec2::new(5.0, 5.5)), ..default() },
            transform: Transform::from_xyz(-29.0, board_y + 3.0, 0.02)
                .with_rotation(Quat::from_rotation_z(-0.3)),
            ..default()
        },
        PlayerPart,
    ));

    // Bindings
    for x in [-9.0, 9.0] {
        // Base plate
        parent.spawn((
            SpriteBundle {
                sprite: Sprite { color: Color::srgb(0.12, 0.12, 0.18), custom_size: Some(Vec2::new(7.0, 9.0)), ..default() },
                transform: Transform::from_xyz(x, board_y + 0.5, 0.03), ..default()
            },
            PlayerPart,
        ));
        // Heel cup
        parent.spawn((
            SpriteBundle {
                sprite: Sprite { color: Color::srgb(0.2, 0.2, 0.28), custom_size: Some(Vec2::new(9.0, 3.0)), ..default() },
                transform: Transform::from_xyz(x, board_y - 2.0, 0.04), ..default()
            },
            PlayerPart,
        ));
        // Strap
        parent.spawn((
            SpriteBundle {
                sprite: Sprite { color: Color::srgb(0.35, 0.35, 0.4), custom_size: Some(Vec2::new(9.0, 2.0)), ..default() },
                transform: Transform::from_xyz(x, board_y + 2.0, 0.04), ..default()
            },
            PlayerPart,
        ));
    }

    // Center stripe design
    parent.spawn((
        SpriteBundle {
            sprite: Sprite { color: Color::srgb(0.95, 0.85, 0.15), custom_size: Some(Vec2::new(16.0, 1.5)), ..default() },
            transform: Transform::from_xyz(0.0, board_y, 0.02), ..default()
        },
        PlayerPart,
    ));
    // Secondary stripe
    parent.spawn((
        SpriteBundle {
            sprite: Sprite { color: accent, custom_size: Some(Vec2::new(10.0, 1.0)), ..default() },
            transform: Transform::from_xyz(0.0, board_y - 2.0, 0.02), ..default()
        },
        PlayerPart,
    ));
}

fn spawn_body(parent: &mut ChildBuilder) {
    let jacket = COLOR_PLAYER_BODY;
    let pants = Color::srgb(0.18, 0.22, 0.32);
    let boots = Color::srgb(0.12, 0.12, 0.16);
    let skin = COLOR_PLAYER_HEAD;
    let glove = Color::srgb(0.15, 0.15, 0.2);

    // Snow boots
    for x in [-7.0, 7.0] {
        parent.spawn((
            SpriteBundle {
                sprite: Sprite { color: boots, custom_size: Some(Vec2::new(8.0, 7.0)), ..default() },
                transform: Transform::from_xyz(x, -16.0, 0.05), ..default()
            },
            PlayerPart,
        ));
        // Boot sole
        parent.spawn((
            SpriteBundle {
                sprite: Sprite { color: Color::srgb(0.08, 0.08, 0.1), custom_size: Some(Vec2::new(9.0, 2.0)), ..default() },
                transform: Transform::from_xyz(x, -19.0, 0.06), ..default()
            },
            PlayerPart,
        ));
    }

    // Legs (pants)
    for x in [-6.0, 6.0] {
        parent.spawn((
            SpriteBundle {
                sprite: Sprite { color: pants, custom_size: Some(Vec2::new(8.0, 16.0)), ..default() },
                transform: Transform::from_xyz(x, -6.0, 0.1), ..default()
            },
            PlayerPart,
        ));
    }

    // Body / jacket
    parent.spawn((
        SpriteBundle {
            sprite: Sprite { color: jacket, custom_size: Some(Vec2::new(24.0, 24.0)), ..default() },
            transform: Transform::from_xyz(0.0, 10.0, 0.2), ..default()
        },
        PlayerPart,
    ));
    // Jacket collar
    parent.spawn((
        SpriteBundle {
            sprite: Sprite { color: Color::srgb(0.8, 0.22, 0.15), custom_size: Some(Vec2::new(20.0, 4.0)), ..default() },
            transform: Transform::from_xyz(0.0, 21.0, 0.25), ..default()
        },
        PlayerPart,
    ));
    // Jacket chest stripe
    parent.spawn((
        SpriteBundle {
            sprite: Sprite { color: Color::srgb(0.95, 0.85, 0.15), custom_size: Some(Vec2::new(24.0, 3.0)), ..default() },
            transform: Transform::from_xyz(0.0, 8.0, 0.25), ..default()
        },
        PlayerPart,
    ));
    // Jacket lower stripe
    parent.spawn((
        SpriteBundle {
            sprite: Sprite { color: Color::srgb(0.95, 0.85, 0.15), custom_size: Some(Vec2::new(24.0, 2.0)), ..default() },
            transform: Transform::from_xyz(0.0, 2.0, 0.25), ..default()
        },
        PlayerPart,
    ));
    // Jacket pocket detail (left)
    parent.spawn((
        SpriteBundle {
            sprite: Sprite { color: Color::srgb(0.75, 0.2, 0.12), custom_size: Some(Vec2::new(6.0, 5.0)), ..default() },
            transform: Transform::from_xyz(-6.0, 13.0, 0.26), ..default()
        },
        PlayerPart,
    ));

    // Arms (upper)
    for (x, rot) in [(-15.0, 0.2_f32), (15.0, -0.2)] {
        parent.spawn((
            SpriteBundle {
                sprite: Sprite { color: jacket, custom_size: Some(Vec2::new(7.0, 14.0)), ..default() },
                transform: Transform::from_xyz(x, 12.0, 0.19)
                    .with_rotation(Quat::from_rotation_z(rot)),
                ..default()
            },
            PlayerPart,
        ));
    }
    // Forearms
    for (x, rot) in [(-17.0, 0.35_f32), (17.0, -0.35)] {
        parent.spawn((
            SpriteBundle {
                sprite: Sprite { color: jacket, custom_size: Some(Vec2::new(6.0, 10.0)), ..default() },
                transform: Transform::from_xyz(x, 4.0, 0.19)
                    .with_rotation(Quat::from_rotation_z(rot)),
                ..default()
            },
            PlayerPart,
        ));
    }
    // Gloves
    for x in [-19.0, 19.0] {
        parent.spawn((
            SpriteBundle {
                sprite: Sprite { color: glove, custom_size: Some(Vec2::new(6.0, 5.0)), ..default() },
                transform: Transform::from_xyz(x, -1.0, 0.21), ..default()
            },
            PlayerPart,
        ));
    }

    // Neck / scarf
    parent.spawn((
        SpriteBundle {
            sprite: Sprite { color: Color::srgb(0.2, 0.6, 0.3), custom_size: Some(Vec2::new(12.0, 5.0)), ..default() },
            transform: Transform::from_xyz(0.0, 24.0, 0.28), ..default()
        },
        PlayerPart,
    ));
    // Scarf tail
    parent.spawn((
        SpriteBundle {
            sprite: Sprite { color: Color::srgb(0.2, 0.6, 0.3), custom_size: Some(Vec2::new(4.0, 10.0)), ..default() },
            transform: Transform::from_xyz(8.0, 18.0, 0.27)
                .with_rotation(Quat::from_rotation_z(-0.3)),
            ..default()
        },
        PlayerPart,
    ));

    // Head
    parent.spawn((
        SpriteBundle {
            sprite: Sprite { color: skin, custom_size: Some(Vec2::new(16.0, 16.0)), ..default() },
            transform: Transform::from_xyz(0.0, 32.0, 0.3), ..default()
        },
        PlayerPart,
    ));
    // Cheeks (subtle blush)
    for x in [-5.0, 5.0] {
        parent.spawn((
            SpriteBundle {
                sprite: Sprite { color: Color::srgba(0.95, 0.6, 0.55, 0.35), custom_size: Some(Vec2::new(4.0, 3.0)), ..default() },
                transform: Transform::from_xyz(x, 30.0, 0.31), ..default()
            },
            PlayerPart,
        ));
    }

    // Goggles frame
    parent.spawn((
        SpriteBundle {
            sprite: Sprite { color: Color::srgb(0.08, 0.08, 0.12), custom_size: Some(Vec2::new(16.0, 6.0)), ..default() },
            transform: Transform::from_xyz(0.0, 34.0, 0.35), ..default()
        },
        PlayerPart,
    ));
    // Goggle lenses (two separate)
    for (x, tint) in [(-3.5, Color::srgb(0.2, 0.6, 0.9)), (3.5, Color::srgb(0.25, 0.65, 0.95))] {
        parent.spawn((
            SpriteBundle {
                sprite: Sprite { color: tint, custom_size: Some(Vec2::new(6.0, 4.0)), ..default() },
                transform: Transform::from_xyz(x, 34.0, 0.36), ..default()
            },
            PlayerPart,
        ));
    }
    // Goggle lens shine
    parent.spawn((
        SpriteBundle {
            sprite: Sprite { color: Color::srgba(1.0, 1.0, 1.0, 0.4), custom_size: Some(Vec2::new(3.0, 2.0)), ..default() },
            transform: Transform::from_xyz(-4.0, 35.0, 0.37), ..default()
        },
        PlayerPart,
    ));
    // Goggle strap
    parent.spawn((
        SpriteBundle {
            sprite: Sprite { color: Color::srgb(0.08, 0.08, 0.12), custom_size: Some(Vec2::new(2.0, 4.0)), ..default() },
            transform: Transform::from_xyz(-9.0, 34.0, 0.29), ..default()
        },
        PlayerPart,
    ));
    parent.spawn((
        SpriteBundle {
            sprite: Sprite { color: Color::srgb(0.08, 0.08, 0.12), custom_size: Some(Vec2::new(2.0, 4.0)), ..default() },
            transform: Transform::from_xyz(9.0, 34.0, 0.29), ..default()
        },
        PlayerPart,
    ));

    // Beanie hat
    parent.spawn((
        SpriteBundle {
            sprite: Sprite { color: Color::srgb(0.75, 0.15, 0.1), custom_size: Some(Vec2::new(18.0, 8.0)), ..default() },
            transform: Transform::from_xyz(0.0, 40.0, 0.3), ..default()
        },
        PlayerPart,
    ));
    // Hat fold/brim
    parent.spawn((
        SpriteBundle {
            sprite: Sprite { color: Color::srgb(0.85, 0.2, 0.15), custom_size: Some(Vec2::new(19.0, 3.0)), ..default() },
            transform: Transform::from_xyz(0.0, 37.0, 0.32), ..default()
        },
        PlayerPart,
    ));
    // Hat top (tapered)
    parent.spawn((
        SpriteBundle {
            sprite: Sprite { color: Color::srgb(0.7, 0.12, 0.08), custom_size: Some(Vec2::new(12.0, 5.0)), ..default() },
            transform: Transform::from_xyz(0.0, 44.0, 0.3), ..default()
        },
        PlayerPart,
    ));
    // Pom-pom
    parent.spawn((
        SpriteBundle {
            sprite: Sprite { color: Color::WHITE, custom_size: Some(Vec2::new(9.0, 9.0)), ..default() },
            transform: Transform::from_xyz(0.0, 48.0, 0.35), ..default()
        },
        PlayerPart,
    ));
    // Pom-pom highlight
    parent.spawn((
        SpriteBundle {
            sprite: Sprite { color: Color::srgba(0.8, 0.85, 0.95, 0.6), custom_size: Some(Vec2::new(4.0, 4.0)), ..default() },
            transform: Transform::from_xyz(-1.0, 49.0, 0.36), ..default()
        },
        PlayerPart,
    ));
}
