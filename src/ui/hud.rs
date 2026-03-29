use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;

pub fn spawn_hud(commands: &mut Commands) {
    // Score text
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Score: 0",
                TextStyle { font_size: 28.0, color: Color::srgb(0.1, 0.1, 0.2), ..default() },
            )
            .with_justify(JustifyText::Left),
            transform: Transform::from_xyz(-WINDOW_WIDTH / 2.0 + 20.0, WINDOW_HEIGHT / 2.0 - 25.0, 51.0),
            text_anchor: bevy::sprite::Anchor::CenterLeft,
            ..default()
        },
        HudUI,
        ScoreText,
        InGame,
    ));

    // Speed text
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Speed: 100%",
                TextStyle { font_size: 28.0, color: Color::srgb(0.1, 0.1, 0.2), ..default() },
            )
            .with_justify(JustifyText::Right),
            transform: Transform::from_xyz(WINDOW_WIDTH / 2.0 - 20.0, WINDOW_HEIGHT / 2.0 - 25.0, 51.0),
            text_anchor: bevy::sprite::Anchor::CenterRight,
            ..default()
        },
        HudUI,
        SpeedText,
        InGame,
    ));
}
