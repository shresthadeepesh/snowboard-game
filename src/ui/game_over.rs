use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::persistence;
use crate::resources::{GameData, PersistentData};
use crate::state::GameState;

const PANEL_W: f32 = 440.0;
const PANEL_H: f32 = 580.0;
const Z_BG: f32 = 100.0;
const Z_PANEL: f32 = 101.0;
const Z_TEXT: f32 = 102.0;

const COLOR_PANEL: Color = Color::srgb(0.06, 0.08, 0.18);
const COLOR_BORDER: Color = Color::srgb(0.2, 0.35, 0.65);
const COLOR_DIVIDER: Color = Color::srgba(0.4, 0.55, 0.8, 0.3);
const COLOR_LABEL: Color = Color::srgb(0.5, 0.6, 0.72);
const COLOR_VALUE: Color = Color::srgb(0.88, 0.92, 0.98);
const COLOR_GOLD: Color = Color::srgb(1.0, 0.85, 0.2);
const COLOR_RED: Color = Color::srgb(1.0, 0.35, 0.25);
const COLOR_PROMPT: Color = Color::srgb(0.55, 0.65, 0.8);

pub fn setup_game_over(
    mut commands: Commands,
    game_data: Res<GameData>,
    mut persistent: ResMut<PersistentData>,
) {
    // Save high score
    if game_data.high_score > persistent.high_score {
        persistent.high_score = game_data.high_score;
        persistence::write_save(&persistence::SaveData {
            high_score: game_data.high_score,
        });
    }

    let is_new_best = game_data.score as u32 >= game_data.high_score && game_data.high_score > 0;

    // ── Dark overlay ──
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.0, 0.0, 0.05, 0.88),
                custom_size: Some(Vec2::new(WINDOW_WIDTH * 2.0, WINDOW_HEIGHT * 2.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, Z_BG),
            ..default()
        },
        GameOverUI,
    ));

    // ── Panel border (outer glow) ──
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: COLOR_BORDER,
                custom_size: Some(Vec2::new(PANEL_W + 6.0, PANEL_H + 6.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, Z_PANEL - 0.2),
            ..default()
        },
        GameOverUI,
    ));

    // ── Panel background ──
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: COLOR_PANEL,
                custom_size: Some(Vec2::new(PANEL_W, PANEL_H)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, Z_PANEL),
            ..default()
        },
        GameOverUI,
    ));

    // ── Inner accent line at top ──
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: COLOR_RED,
                custom_size: Some(Vec2::new(PANEL_W - 40.0, 3.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, PANEL_H / 2.0 - 20.0, Z_PANEL + 0.1),
            ..default()
        },
        GameOverUI,
    ));

    let mut y = PANEL_H / 2.0 - 55.0;

    // ── Title ──
    spawn_text(&mut commands, "WIPEOUT!", 0.0, y, 54.0, COLOR_RED);
    y -= 55.0;

    // ── NEW BEST label ──
    if is_new_best {
        spawn_text(&mut commands, "NEW BEST!", 0.0, y + 10.0, 22.0, COLOR_GOLD);
        y -= 15.0;
    }

    // ── Score ──
    spawn_text(
        &mut commands,
        &format!("{}", game_data.score as u32),
        0.0,
        y,
        48.0,
        Color::WHITE,
    );
    y -= 35.0;
    spawn_text(&mut commands, "SCORE", 0.0, y, 14.0, COLOR_LABEL);
    y -= 25.0;

    // ── Divider ──
    spawn_divider(&mut commands, y);
    y -= 25.0;

    // ── Best score ──
    spawn_text(
        &mut commands,
        &format!("Best: {}", game_data.high_score),
        0.0,
        y,
        24.0,
        COLOR_GOLD,
    );
    y -= 40.0;

    // ── Divider ──
    spawn_divider(&mut commands, y);
    y -= 30.0;

    // ── Stats grid (2 columns) ──
    let left_x = -100.0;
    let right_x = 100.0;
    let row_h = 42.0;

    // Row 1: Distance | Max Speed
    let speed_pct = ((game_data.scroll_speed / BASE_SCROLL_SPEED) * 100.0) as u32;
    spawn_stat(&mut commands, "Distance", &format!("{:.0}m", game_data.distance / 10.0), left_x, y);
    spawn_stat(&mut commands, "Max Speed", &format!("{}%", speed_pct), right_x, y);
    y -= row_h;

    // Row 2: Dodged | Near Misses
    spawn_stat(&mut commands, "Dodged", &format!("{}", game_data.obstacles_dodged), left_x, y);
    spawn_stat(&mut commands, "Near Misses", &format!("{}", game_data.near_misses), right_x, y);
    y -= row_h;

    // Row 3: Coins | Best Combo
    spawn_stat(&mut commands, "Coins", &format!("{}", game_data.coins_collected), left_x, y);
    spawn_stat(&mut commands, "Best Combo", &format!("x{}", game_data.max_combo), right_x, y);
    y -= row_h;

    // ── Divider ──
    spawn_divider(&mut commands, y + 15.0);
    y -= 15.0;

    // ── Difficulty label ──
    spawn_text(
        &mut commands,
        &format!("Difficulty: {}", game_data.difficulty.label()),
        0.0,
        y,
        16.0,
        COLOR_LABEL,
    );
    y -= 45.0;

    // ── Controls prompt ──
    spawn_text(&mut commands, "SPACE / R", -80.0, y, 20.0, COLOR_VALUE);
    spawn_text(&mut commands, "Retry", -80.0, y - 22.0, 14.0, COLOR_PROMPT);
    spawn_text(&mut commands, "ESC", 80.0, y, 20.0, COLOR_VALUE);
    spawn_text(&mut commands, "Menu", 80.0, y - 22.0, 14.0, COLOR_PROMPT);
}

fn spawn_text(commands: &mut Commands, content: &str, x: f32, y: f32, size: f32, color: Color) {
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(content, TextStyle { font_size: size, color, ..default() })
                .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(x, y, Z_TEXT),
            ..default()
        },
        GameOverUI,
    ));
}

fn spawn_stat(commands: &mut Commands, label: &str, value: &str, x: f32, y: f32) {
    // Value (larger, brighter)
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(value, TextStyle { font_size: 22.0, color: COLOR_VALUE, ..default() })
                .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(x, y, Z_TEXT),
            ..default()
        },
        GameOverUI,
    ));
    // Label (smaller, dimmer, below)
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(label, TextStyle { font_size: 12.0, color: COLOR_LABEL, ..default() })
                .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(x, y - 16.0, Z_TEXT),
            ..default()
        },
        GameOverUI,
    ));
}

fn spawn_divider(commands: &mut Commands, y: f32) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: COLOR_DIVIDER,
                custom_size: Some(Vec2::new(340.0, 1.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, y, Z_PANEL + 0.1),
            ..default()
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
