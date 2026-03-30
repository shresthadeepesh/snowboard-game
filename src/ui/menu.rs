use bevy::prelude::*;
use crate::components::MenuUI;
use crate::constants::*;
use crate::resources::{DifficultyMode, GameData, MenuScreenKind, MenuState, PersistentData, Settings};
use crate::persistence;
use crate::state::GameState;

// --- Colors ---
const MENU_BG: Color = Color::srgba(0.08, 0.12, 0.22, 1.0);
const MENU_PANEL: Color = Color::srgba(0.12, 0.18, 0.32, 0.95);
const MENU_ACCENT: Color = Color::srgb(0.3, 0.7, 1.0);
const MENU_TEXT: Color = Color::srgb(0.88, 0.92, 0.98);
const MENU_TEXT_DIM: Color = Color::srgb(0.5, 0.55, 0.65);
const MENU_HIGHLIGHT: Color = Color::srgb(1.0, 0.84, 0.0);
const MENU_BTN_SELECTED: Color = Color::srgba(0.3, 0.5, 0.85, 1.0);
const MENU_SEPARATOR: Color = Color::srgba(0.3, 0.4, 0.6, 0.4);
const DIFF_EASY: Color = Color::srgb(0.3, 0.85, 0.4);
const DIFF_NORMAL: Color = Color::srgb(1.0, 0.75, 0.2);
const DIFF_HARD: Color = Color::srgb(1.0, 0.3, 0.3);

const MAIN_MENU_COUNT: usize = 3;
const SETTINGS_MENU_COUNT: usize = 7;

// ─── OnEnter: load persistence + initial render ───

pub fn setup_menu(
    mut commands: Commands,
    mut game_data: ResMut<GameData>,
    mut persistent: ResMut<PersistentData>,
    mut menu_state: ResMut<MenuState>,
    mut settings: ResMut<Settings>,
    existing: Query<Entity, With<MenuUI>>,
) {
    // Load persistent data on first menu enter
    if !persistent.loaded {
        let save = persistence::load_save();
        persistent.high_score = save.high_score;
        settings.master_volume = save.master_volume;
        settings.music_volume = save.music_volume;
        settings.sfx_volume = save.sfx_volume;
        game_data.difficulty = match save.difficulty {
            0 => DifficultyMode::Easy,
            2 => DifficultyMode::Hard,
            _ => DifficultyMode::Normal,
        };
        persistent.loaded = true;
    }
    game_data.high_score = game_data.high_score.max(persistent.high_score);

    // Reset cursor when entering menu from another state
    menu_state.cursor = 0;
    menu_state.screen = MenuScreenKind::Main;

    // Clear any leftover menu entities
    for entity in &existing {
        commands.entity(entity).despawn_recursive();
    }

    render_menu(&mut commands, &game_data, &settings, &menu_state);
}

// ─── Update: rebuild when MenuState, Settings, or GameData change ───

pub fn rebuild_menu(
    mut commands: Commands,
    menu_state: Res<MenuState>,
    settings: Res<Settings>,
    game_data: Res<GameData>,
    existing: Query<Entity, With<MenuUI>>,
) {
    if !menu_state.is_changed() && !settings.is_changed() && !game_data.is_changed() {
        return;
    }
    // Skip the initial add (setup_menu already rendered)
    if menu_state.is_added() {
        return;
    }

    for entity in &existing {
        commands.entity(entity).despawn_recursive();
    }
    render_menu(&mut commands, &game_data, &settings, &menu_state);
}

// ─── Shared render function ───

fn render_menu(commands: &mut Commands, game_data: &GameData, settings: &Settings, menu_state: &MenuState) {
    // Dark background
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: MENU_BG,
                custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, -1.0),
            ..default()
        },
        MenuUI,
    ));

    // Mountain silhouettes
    for (x, w, h, c) in [
        (-220.0, 400.0, 280.0, Color::srgba(0.1, 0.14, 0.26, 0.8)),
        (20.0, 500.0, 360.0, Color::srgba(0.09, 0.13, 0.24, 0.9)),
        (250.0, 350.0, 220.0, Color::srgba(0.11, 0.15, 0.28, 0.7)),
    ] {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite { color: c, custom_size: Some(Vec2::new(w, h)), ..default() },
                transform: Transform::from_xyz(x, -250.0, -0.5),
                ..default()
            },
            MenuUI,
        ));
    }

    // Snow caps
    for (x, w) in [(-220.0, 220.0), (20.0, 280.0), (250.0, 190.0)] {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.7, 0.8, 0.95, 0.3),
                    custom_size: Some(Vec2::new(w, 20.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, -100.0, -0.4),
                ..default()
            },
            MenuUI,
        ));
    }

    match menu_state.screen {
        MenuScreenKind::Main => spawn_main_menu(commands, game_data, menu_state.cursor),
        MenuScreenKind::Settings => spawn_settings_menu(commands, game_data, settings, menu_state.cursor),
    }
}

// ─── Main Menu ───

fn spawn_main_menu(commands: &mut Commands, game_data: &GameData, cursor: usize) {
    // Title
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("SNOW RIDER", TextStyle {
                font_size: 78.0, color: MENU_ACCENT, ..default()
            }).with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, 320.0, 1.0),
            ..default()
        },
        MenuUI,
    ));

    // Subtitle
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Escape the Mountain!", TextStyle {
                font_size: 24.0, color: MENU_TEXT_DIM, ..default()
            }).with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, 265.0, 1.0),
            ..default()
        },
        MenuUI,
    ));

    // Decorative line
    commands.spawn((
        SpriteBundle {
            sprite: Sprite { color: MENU_ACCENT, custom_size: Some(Vec2::new(200.0, 2.0)), ..default() },
            transform: Transform::from_xyz(0.0, 245.0, 1.0),
            ..default()
        },
        MenuUI,
    ));

    // Snowboarder
    spawn_menu_snowboarder(commands, Vec3::new(0.0, 145.0, 1.0), 2.2);

    // Central panel
    commands.spawn((
        SpriteBundle {
            sprite: Sprite { color: MENU_PANEL, custom_size: Some(Vec2::new(420.0, 260.0)), ..default() },
            transform: Transform::from_xyz(0.0, -55.0, 0.5),
            ..default()
        },
        MenuUI,
    ));

    // Panel top accent
    commands.spawn((
        SpriteBundle {
            sprite: Sprite { color: MENU_ACCENT, custom_size: Some(Vec2::new(420.0, 3.0)), ..default() },
            transform: Transform::from_xyz(0.0, 75.0, 0.6),
            ..default()
        },
        MenuUI,
    ));

    // Menu items
    let items: [(&str, Color); 3] = [
        ("PLAY", Color::WHITE),
        ("SETTINGS", MENU_TEXT),
        ("EXIT", Color::srgb(1.0, 0.45, 0.45)),
    ];

    let start_y = 40.0;
    let spacing = 60.0;

    for (i, (label, base_color)) in items.iter().enumerate() {
        let y = start_y - i as f32 * spacing;
        let is_selected = cursor == i;

        // Selection highlight background (only for the focused item)
        if is_selected {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite { color: MENU_BTN_SELECTED, custom_size: Some(Vec2::new(300.0, 46.0)), ..default() },
                    transform: Transform::from_xyz(0.0, y, 0.65),
                    ..default()
                },
                MenuUI,
            ));
        }

        let text_color = if is_selected { *base_color } else {
            Color::srgba(
                base_color.to_srgba().red * 0.7,
                base_color.to_srgba().green * 0.7,
                base_color.to_srgba().blue * 0.7,
                1.0,
            )
        };

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(*label, TextStyle {
                    font_size: if i == 0 { 36.0 } else { 28.0 },
                    color: text_color,
                    ..default()
                }).with_justify(JustifyText::Center),
                transform: Transform::from_xyz(0.0, y, 0.8),
                ..default()
            },
            MenuUI,
        ));

        if i < items.len() - 1 {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite { color: MENU_SEPARATOR, custom_size: Some(Vec2::new(340.0, 1.0)), ..default() },
                    transform: Transform::from_xyz(0.0, y - spacing / 2.0, 0.6),
                    ..default()
                },
                MenuUI,
            ));
        }
    }

    // Difficulty display
    let diff_color = match game_data.difficulty {
        DifficultyMode::Easy => DIFF_EASY,
        DifficultyMode::Normal => DIFF_NORMAL,
        DifficultyMode::Hard => DIFF_HARD,
    };
    commands.spawn((
        Text2dBundle {
            text: Text::from_sections([
                TextSection::new("Difficulty: ", TextStyle { font_size: 20.0, color: MENU_TEXT_DIM, ..default() }),
                TextSection::new(game_data.difficulty.label(), TextStyle { font_size: 20.0, color: diff_color, ..default() }),
            ]).with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, -215.0, 1.0),
            ..default()
        },
        MenuUI,
    ));

    // High score
    if game_data.high_score > 0 {
        commands.spawn((
            Text2dBundle {
                text: Text::from_sections([
                    TextSection::new("Best: ", TextStyle { font_size: 22.0, color: MENU_TEXT_DIM, ..default() }),
                    TextSection::new(format!("{}", game_data.high_score), TextStyle { font_size: 22.0, color: MENU_HIGHLIGHT, ..default() }),
                ]).with_justify(JustifyText::Center),
                transform: Transform::from_xyz(0.0, -250.0, 1.0),
                ..default()
            },
            MenuUI,
        ));
    }

    // Controls hint
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Up/Down to Navigate  |  Enter to Select", TextStyle {
                font_size: 15.0, color: Color::srgba(0.4, 0.5, 0.65, 0.7), ..default()
            }).with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, -340.0, 1.0),
            ..default()
        },
        MenuUI,
    ));
}

// ─── Settings Menu ───

fn spawn_settings_menu(commands: &mut Commands, game_data: &GameData, settings: &Settings, cursor: usize) {
    // Title
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("SETTINGS", TextStyle {
                font_size: 56.0, color: MENU_ACCENT, ..default()
            }).with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, 330.0, 1.0),
            ..default()
        },
        MenuUI,
    ));

    // Decorative line
    commands.spawn((
        SpriteBundle {
            sprite: Sprite { color: MENU_ACCENT, custom_size: Some(Vec2::new(180.0, 2.0)), ..default() },
            transform: Transform::from_xyz(0.0, 300.0, 1.0),
            ..default()
        },
        MenuUI,
    ));

    // Settings panel
    commands.spawn((
        SpriteBundle {
            sprite: Sprite { color: MENU_PANEL, custom_size: Some(Vec2::new(520.0, 500.0)), ..default() },
            transform: Transform::from_xyz(0.0, 10.0, 0.5),
            ..default()
        },
        MenuUI,
    ));

    // Panel top accent
    commands.spawn((
        SpriteBundle {
            sprite: Sprite { color: MENU_ACCENT, custom_size: Some(Vec2::new(520.0, 3.0)), ..default() },
            transform: Transform::from_xyz(0.0, 260.0, 0.6),
            ..default()
        },
        MenuUI,
    ));

    let left_x = -220.0;
    let bar_x = 80.0;

    // --- AUDIO SECTION ---
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("AUDIO", TextStyle {
                font_size: 26.0, color: MENU_ACCENT, ..default()
            }).with_justify(JustifyText::Left),
            text_anchor: bevy::sprite::Anchor::CenterLeft,
            transform: Transform::from_xyz(left_x, 230.0, 0.8),
            ..default()
        },
        MenuUI,
    ));

    spawn_settings_volume_row(commands, "Master Volume", settings.master_volume, left_x, bar_x, 185.0, cursor == 0);
    spawn_settings_volume_row(commands, "Music Volume", settings.music_volume, left_x, bar_x, 135.0, cursor == 1);
    spawn_settings_volume_row(commands, "SFX Volume", settings.sfx_volume, left_x, bar_x, 85.0, cursor == 2);

    // Separator
    commands.spawn((
        SpriteBundle {
            sprite: Sprite { color: MENU_SEPARATOR, custom_size: Some(Vec2::new(460.0, 1.0)), ..default() },
            transform: Transform::from_xyz(0.0, 40.0, 0.6),
            ..default()
        },
        MenuUI,
    ));

    // --- DIFFICULTY SECTION ---
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("DIFFICULTY", TextStyle {
                font_size: 26.0, color: MENU_ACCENT, ..default()
            }).with_justify(JustifyText::Left),
            text_anchor: bevy::sprite::Anchor::CenterLeft,
            transform: Transform::from_xyz(left_x, 5.0, 0.8),
            ..default()
        },
        MenuUI,
    ));

    let diff_items: [(DifficultyMode, &str, Color); 3] = [
        (DifficultyMode::Easy, "EASY", DIFF_EASY),
        (DifficultyMode::Normal, "NORMAL", DIFF_NORMAL),
        (DifficultyMode::Hard, "HARD", DIFF_HARD),
    ];

    for (i, (mode, label, color)) in diff_items.iter().enumerate() {
        let y = -40.0 - i as f32 * 45.0;
        let menu_idx = 3 + i;
        let is_cursor = cursor == menu_idx;
        let is_active = game_data.difficulty == *mode;

        if is_cursor {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(color.to_srgba().red, color.to_srgba().green, color.to_srgba().blue, 0.15),
                        custom_size: Some(Vec2::new(460.0, 38.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, y, 0.6),
                    ..default()
                },
                MenuUI,
            ));
        }

        let prefix = if is_active { "* " } else { "  " };
        let text_color = if is_active {
            *color
        } else if is_cursor {
            MENU_TEXT
        } else {
            MENU_TEXT_DIM
        };

        commands.spawn((
            Text2dBundle {
                text: Text::from_section(format!("{}{}", prefix, label), TextStyle {
                    font_size: 24.0, color: text_color, ..default()
                }).with_justify(JustifyText::Left),
                text_anchor: bevy::sprite::Anchor::CenterLeft,
                transform: Transform::from_xyz(left_x, y, 0.8),
                ..default()
            },
            MenuUI,
        ));
    }

    // Separator before Back
    commands.spawn((
        SpriteBundle {
            sprite: Sprite { color: MENU_SEPARATOR, custom_size: Some(Vec2::new(460.0, 1.0)), ..default() },
            transform: Transform::from_xyz(0.0, -190.0, 0.6),
            ..default()
        },
        MenuUI,
    ));

    // Item 6: BACK
    let back_y = -225.0;
    let is_back_selected = cursor == 6;

    if is_back_selected {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.2, 0.3, 0.5, 0.5),
                    custom_size: Some(Vec2::new(200.0, 42.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, back_y, 0.6),
                ..default()
            },
            MenuUI,
        ));
    }

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("BACK", TextStyle {
                font_size: 30.0,
                color: if is_back_selected { MENU_TEXT } else { MENU_TEXT_DIM },
                ..default()
            }).with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, back_y, 0.8),
            ..default()
        },
        MenuUI,
    ));

    // Controls hint
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Up/Down Navigate  |  Left/Right Adjust  |  Enter Select", TextStyle {
                font_size: 14.0, color: Color::srgba(0.4, 0.5, 0.65, 0.7), ..default()
            }).with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, -310.0, 1.0),
            ..default()
        },
        MenuUI,
    ));
}

fn spawn_settings_volume_row(
    commands: &mut Commands,
    label: &str,
    value: f32,
    label_x: f32,
    bar_x: f32,
    y: f32,
    is_selected: bool,
) {
    if is_selected {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.2, 0.35, 0.55, 0.3),
                    custom_size: Some(Vec2::new(460.0, 38.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, y, 0.6),
                ..default()
            },
            MenuUI,
        ));
    }

    let text_color = if is_selected { MENU_TEXT } else { MENU_TEXT_DIM };
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(label, TextStyle {
                font_size: 22.0, color: text_color, ..default()
            }).with_justify(JustifyText::Left),
            text_anchor: bevy::sprite::Anchor::CenterLeft,
            transform: Transform::from_xyz(label_x, y, 0.8),
            ..default()
        },
        MenuUI,
    ));

    let pct = (value * 100.0).round() as u32;
    let bar_width = 140.0;
    let filled_width = bar_width * value;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.2, 0.25, 0.35, 0.8),
                custom_size: Some(Vec2::new(bar_width, 14.0)),
                ..default()
            },
            transform: Transform::from_xyz(bar_x + bar_width / 2.0, y, 0.7),
            ..default()
        },
        MenuUI,
    ));

    if filled_width > 0.5 {
        let bar_color = if is_selected { MENU_ACCENT } else { Color::srgb(0.25, 0.5, 0.75) };
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: bar_color,
                    custom_size: Some(Vec2::new(filled_width, 14.0)),
                    ..default()
                },
                transform: Transform::from_xyz(bar_x + filled_width / 2.0, y, 0.75),
                ..default()
            },
            MenuUI,
        ));
    }

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(format!("{}%", pct), TextStyle {
                font_size: 18.0, color: text_color, ..default()
            }).with_justify(JustifyText::Left),
            text_anchor: bevy::sprite::Anchor::CenterLeft,
            transform: Transform::from_xyz(bar_x + bar_width + 12.0, y, 0.8),
            ..default()
        },
        MenuUI,
    ));
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

// ─── Input: only mutates resources, no state transitions for navigation ───

pub fn menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut game_data: ResMut<GameData>,
    mut menu_state: ResMut<MenuState>,
    mut settings: ResMut<Settings>,
    mut exit: EventWriter<AppExit>,
) {
    match menu_state.screen {
        MenuScreenKind::Main => {
            if keyboard.just_pressed(KeyCode::ArrowUp) {
                if menu_state.cursor > 0 {
                    menu_state.cursor -= 1;
                } else {
                    menu_state.cursor = MAIN_MENU_COUNT - 1;
                }
                return;
            }
            if keyboard.just_pressed(KeyCode::ArrowDown) {
                menu_state.cursor = (menu_state.cursor + 1) % MAIN_MENU_COUNT;
                return;
            }

            if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
                match menu_state.cursor {
                    0 => {
                        save_settings(&game_data, &settings);
                        next_state.set(GameState::Playing);
                    }
                    1 => {
                        menu_state.screen = MenuScreenKind::Settings;
                        menu_state.cursor = 0;
                    }
                    2 => {
                        save_settings(&game_data, &settings);
                        exit.send(AppExit::Success);
                    }
                    _ => {}
                }
            }
        }
        MenuScreenKind::Settings => {
            let step = 0.05;

            // Navigate up/down
            if keyboard.just_pressed(KeyCode::ArrowUp) {
                if menu_state.cursor > 0 {
                    menu_state.cursor -= 1;
                } else {
                    menu_state.cursor = SETTINGS_MENU_COUNT - 1;
                }
                return;
            }
            if keyboard.just_pressed(KeyCode::ArrowDown) {
                menu_state.cursor = (menu_state.cursor + 1) % SETTINGS_MENU_COUNT;
                return;
            }

            // Left/Right adjusts volume for items 0-2
            if keyboard.just_pressed(KeyCode::ArrowLeft) {
                match menu_state.cursor {
                    0 => settings.master_volume = (settings.master_volume - step).max(0.0),
                    1 => settings.music_volume = (settings.music_volume - step).max(0.0),
                    2 => settings.sfx_volume = (settings.sfx_volume - step).max(0.0),
                    _ => {}
                }
                return;
            }
            if keyboard.just_pressed(KeyCode::ArrowRight) {
                match menu_state.cursor {
                    0 => settings.master_volume = (settings.master_volume + step).min(1.0),
                    1 => settings.music_volume = (settings.music_volume + step).min(1.0),
                    2 => settings.sfx_volume = (settings.sfx_volume + step).min(1.0),
                    _ => {}
                }
                return;
            }

            // Enter selects difficulty or Back
            if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
                match menu_state.cursor {
                    3 => game_data.difficulty = DifficultyMode::Easy,
                    4 => game_data.difficulty = DifficultyMode::Normal,
                    5 => game_data.difficulty = DifficultyMode::Hard,
                    6 => {
                        save_settings(&game_data, &settings);
                        menu_state.screen = MenuScreenKind::Main;
                        menu_state.cursor = 0;
                    }
                    _ => {}
                }
                return;
            }

            // ESC always goes back
            if keyboard.just_pressed(KeyCode::Escape) {
                save_settings(&game_data, &settings);
                menu_state.screen = MenuScreenKind::Main;
                menu_state.cursor = 0;
            }
        }
    }
}

fn save_settings(game_data: &GameData, settings: &Settings) {
    persistence::write_save(&persistence::SaveData {
        high_score: game_data.high_score,
        master_volume: settings.master_volume,
        music_volume: settings.music_volume,
        sfx_volume: settings.sfx_volume,
        difficulty: match game_data.difficulty {
            DifficultyMode::Easy => 0,
            DifficultyMode::Normal => 1,
            DifficultyMode::Hard => 2,
        },
    });
}
