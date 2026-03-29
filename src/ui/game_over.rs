use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::resources::GameData;
use crate::state::GameState;

pub fn setup_game_over(mut commands: Commands, game_data: Res<GameData>) {
    // Overlay
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.0, 0.0, 0.05, 0.85),
                custom_size: Some(Vec2::new(WINDOW_WIDTH * 2.0, WINDOW_HEIGHT * 2.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            ..default()
        },
        GameOverUI,
    ));

    // Panel
    commands.spawn((
        SpriteBundle {
            sprite: Sprite { color: Color::srgb(0.08, 0.1, 0.22), custom_size: Some(Vec2::new(420.0, 370.0)), ..default() },
            transform: Transform::from_xyz(0.0, 20.0, 101.0), ..default()
        },
        GameOverUI,
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite { color: Color::srgb(0.25, 0.4, 0.7), custom_size: Some(Vec2::new(426.0, 376.0)), ..default() },
            transform: Transform::from_xyz(0.0, 20.0, 100.5), ..default()
        },
        GameOverUI,
    ));

    // Title
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("WIPEOUT!", TextStyle { font_size: 56.0, color: Color::srgb(1.0, 0.35, 0.25), ..default() })
                .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, 140.0, 102.0), ..default()
        },
        GameOverUI,
    ));

    // Score
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(format!("Score: {}", game_data.score as u32), TextStyle { font_size: 40.0, color: Color::WHITE, ..default() })
                .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, 60.0, 102.0), ..default()
        },
        GameOverUI,
        FinalScoreText,
    ));

    // High score
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(format!("Best: {}", game_data.high_score), TextStyle { font_size: 30.0, color: Color::srgb(1.0, 0.85, 0.2), ..default() })
                .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, 10.0, 102.0), ..default()
        },
        GameOverUI,
    ));

    // Distance
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(format!("Distance: {:.0}m", game_data.distance / 10.0), TextStyle { font_size: 24.0, color: Color::srgb(0.75, 0.85, 0.95), ..default() })
                .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, -30.0, 102.0), ..default()
        },
        GameOverUI,
    ));

    // Max speed
    let speed_pct = ((game_data.scroll_speed / BASE_SCROLL_SPEED) * 100.0) as u32;
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(format!("Max Speed: {}%", speed_pct), TextStyle { font_size: 24.0, color: Color::srgb(0.75, 0.85, 0.95), ..default() })
                .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, -65.0, 102.0), ..default()
        },
        GameOverUI,
    ));

    // Prompt
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("SPACE / R  -  Retry   |   ESC  -  Menu", TextStyle { font_size: 22.0, color: Color::srgb(0.7, 0.8, 0.9), ..default() })
                .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, -120.0, 102.0), ..default()
        },
        GameOverUI,
    ));
}

pub fn game_over_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::KeyR) {
        next_state.set(GameState::Playing);
    }
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Menu);
    }
}
