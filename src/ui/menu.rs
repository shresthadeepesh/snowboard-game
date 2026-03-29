use bevy::prelude::*;
use crate::components::MenuUI;
use crate::constants::*;
use crate::resources::GameData;
use crate::state::GameState;

pub fn setup_menu(mut commands: Commands, game_data: Res<GameData>) {
    // Background
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.85, 0.9, 0.95, 1.0),
                custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, -1.0),
            ..default()
        },
        MenuUI,
    ));

    // Mountains
    for (x, w, h, c) in [
        (-200.0, 350.0, 250.0, Color::srgb(0.75, 0.8, 0.88)),
        (0.0, 400.0, 320.0, Color::srgb(0.7, 0.76, 0.85)),
        (200.0, 300.0, 200.0, Color::srgb(0.78, 0.83, 0.9)),
    ] {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite { color: c, custom_size: Some(Vec2::new(w, h)), ..default() },
                transform: Transform::from_xyz(x, -150.0, 0.0),
                ..default()
            },
            MenuUI,
        ));
    }

    // Snow caps
    for (x, w) in [(-200.0, 200.0), (0.0, 250.0), (200.0, 180.0)] {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite { color: Color::WHITE, custom_size: Some(Vec2::new(w, 30.0)), ..default() },
                transform: Transform::from_xyz(x, -30.0, 0.1),
                ..default()
            },
            MenuUI,
        ));
    }

    // Title
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("SNOW RIDER", TextStyle { font_size: 72.0, color: COLOR_TITLE, ..default() })
                .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, 250.0, 1.0),
            ..default()
        },
        MenuUI,
    ));

    // Subtitle
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Escape the Mountain!", TextStyle { font_size: 28.0, color: Color::srgb(0.3, 0.45, 0.65), ..default() })
                .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, 190.0, 1.0),
            ..default()
        },
        MenuUI,
    ));

    // Snowboarder silhouette
    spawn_menu_snowboarder(&mut commands, Vec3::new(0.0, 50.0, 1.0), 2.5);

    // Instructions
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Press SPACE or ENTER to Start", TextStyle { font_size: 30.0, color: Color::srgb(0.2, 0.35, 0.55), ..default() })
                .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, -120.0, 1.0),
            ..default()
        },
        MenuUI,
    ));

    // Controls
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Arrow Keys / A,D to Move  |  ESC to Pause", TextStyle { font_size: 18.0, color: Color::srgb(0.4, 0.5, 0.65), ..default() })
                .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, -170.0, 1.0),
            ..default()
        },
        MenuUI,
    ));

    // High score
    if game_data.high_score > 0 {
        commands.spawn((
            Text2dBundle {
                text: Text::from_section(format!("High Score: {}", game_data.high_score), TextStyle { font_size: 24.0, color: Color::srgb(0.8, 0.6, 0.1), ..default() })
                    .with_justify(JustifyText::Center),
                transform: Transform::from_xyz(0.0, -220.0, 1.0),
                ..default()
            },
            MenuUI,
        ));
    }
}

fn spawn_menu_snowboarder(commands: &mut Commands, pos: Vec3, scale: f32) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite { color: COLOR_PLAYER_BOARD, custom_size: Some(Vec2::new(50.0 * scale, 8.0 * scale)), ..default() },
            transform: Transform::from_translation(pos + Vec3::new(0.0, -20.0 * scale, 0.0))
                .with_rotation(Quat::from_rotation_z(-0.25)),
            ..default()
        },
        MenuUI,
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite { color: COLOR_PLAYER_BODY, custom_size: Some(Vec2::new(18.0 * scale, 30.0 * scale)), ..default() },
            transform: Transform::from_translation(pos + Vec3::new(0.0, 5.0 * scale, 0.1)),
            ..default()
        },
        MenuUI,
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite { color: COLOR_PLAYER_HEAD, custom_size: Some(Vec2::new(14.0 * scale, 14.0 * scale)), ..default() },
            transform: Transform::from_translation(pos + Vec3::new(0.0, 28.0 * scale, 0.1)),
            ..default()
        },
        MenuUI,
    ));
}

pub fn menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::Enter) {
        next_state.set(GameState::Playing);
    }
}
