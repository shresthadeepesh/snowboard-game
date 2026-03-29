use bevy::prelude::*;
use crate::components::PausedUI;
use crate::constants::*;
use crate::resources::GameData;
use crate::state::GameState;

pub fn setup_paused(mut commands: Commands) {
    // Overlay
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.0, 0.0, 0.05, 0.7),
                custom_size: Some(Vec2::new(WINDOW_WIDTH * 2.0, WINDOW_HEIGHT * 2.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            ..default()
        },
        PausedUI,
    ));

    // Panel
    commands.spawn((
        SpriteBundle {
            sprite: Sprite { color: Color::srgb(0.08, 0.1, 0.22), custom_size: Some(Vec2::new(380.0, 280.0)), ..default() },
            transform: Transform::from_xyz(0.0, 20.0, 101.0), ..default()
        },
        PausedUI,
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite { color: Color::srgb(0.25, 0.4, 0.7), custom_size: Some(Vec2::new(386.0, 286.0)), ..default() },
            transform: Transform::from_xyz(0.0, 20.0, 100.5), ..default()
        },
        PausedUI,
    ));

    let text_color = Color::srgb(0.75, 0.85, 0.95);

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("PAUSED", TextStyle { font_size: 52.0, color: Color::WHITE, ..default() })
                .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, 100.0, 102.0), ..default()
        },
        PausedUI,
    ));

    for (label, y) in [("ESC  -  Resume", 30.0), ("R  -  Restart", -10.0), ("Q  -  Quit to Menu", -50.0)] {
        commands.spawn((
            Text2dBundle {
                text: Text::from_section(label, TextStyle { font_size: 26.0, color: text_color, ..default() })
                    .with_justify(JustifyText::Center),
                transform: Transform::from_xyz(0.0, y, 102.0), ..default()
            },
            PausedUI,
        ));
    }
}

pub fn paused_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut game_data: ResMut<GameData>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        game_data.is_paused_resume = true;
        next_state.set(GameState::Playing);
    }
    if keyboard.just_pressed(KeyCode::KeyR) {
        next_state.set(GameState::Playing);
    }
    if keyboard.just_pressed(KeyCode::KeyQ) {
        next_state.set(GameState::Menu);
    }
}
