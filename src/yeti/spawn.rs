use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::resources::{Settings, SfxHandles, SpawnTimers};

pub fn spawn_yeti(
    mut commands: Commands,
    time: Res<Time>,
    mut timers: ResMut<SpawnTimers>,
    player_query: Query<(&Transform, &Player)>,
    sfx: Res<SfxHandles>,
    settings: Res<Settings>,
) {
    if timers.yeti_spawned {
        return;
    }
    if player_query.get_single().map_or(true, |(_, p)| !p.intro_done) {
        return;
    }

    timers.yeti_timer.tick(time.delta());
    if !timers.yeti_timer.just_finished() {
        return;
    }

    timers.yeti_spawned = true;

    // Play yeti roar
    commands.spawn(AudioBundle {
        source: sfx.yeti_roar.clone(),
        settings: PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Despawn,
            volume: bevy::audio::Volume::new(0.6 * settings.effective_sfx()),
            ..default()
        },
    });

    let player_pos = player_query
        .get_single()
        .map(|(t, _)| t.translation)
        .unwrap_or(Vec3::ZERO);
    let target_y = player_pos.y + YETI_START_DISTANCE;
    let spawn_y = WINDOW_HEIGHT / 2.0 + 80.0;

    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::NONE,
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    ..default()
                },
                transform: Transform::from_xyz(player_pos.x, spawn_y, 15.0),
                ..default()
            },
            Yeti { target_y, intro_done: false },
            InGame,
        ))
        .with_children(|parent| {
            spawn_yeti_parts(parent);
        });

    // Warning text
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "!! YETI INCOMING !!",
                TextStyle {
                    font_size: 32.0,
                    color: Color::srgb(0.95, 0.2, 0.15),
                    ..default()
                },
            )
            .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, WINDOW_HEIGHT / 2.0 - 80.0, 55.0),
            ..default()
        },
        YetiWarningText,
        InGame,
    ));
}

fn spawn_yeti_parts(parent: &mut ChildBuilder) {
    // Shadow
    parent.spawn((SpriteBundle {
        sprite: Sprite { color: Color::srgba(0.0, 0.0, 0.0, 0.2), custom_size: Some(Vec2::new(70.0, 20.0)), ..default() },
        transform: Transform::from_xyz(0.0, -40.0, -0.1), ..default()
    }, YetiPart));
    // Feet
    for x in [-14.0, 14.0] {
        parent.spawn((SpriteBundle {
            sprite: Sprite { color: Color::srgb(0.65, 0.62, 0.58), custom_size: Some(Vec2::new(18.0, 14.0)), ..default() },
            transform: Transform::from_xyz(x, -34.0, 0.0), ..default()
        }, YetiPart));
    }
    // Legs
    for x in [-12.0, 12.0] {
        parent.spawn((SpriteBundle {
            sprite: Sprite { color: Color::srgb(0.88, 0.86, 0.84), custom_size: Some(Vec2::new(16.0, 20.0)), ..default() },
            transform: Transform::from_xyz(x, -22.0, 0.05), ..default()
        }, YetiPart));
    }
    // Body
    parent.spawn((SpriteBundle {
        sprite: Sprite { color: Color::srgb(0.92, 0.9, 0.88), custom_size: Some(Vec2::new(56.0, 50.0)), ..default() },
        transform: Transform::from_xyz(0.0, 4.0, 0.1), ..default()
    }, YetiPart));
    // Belly fur
    parent.spawn((SpriteBundle {
        sprite: Sprite { color: Color::srgb(0.96, 0.95, 0.93), custom_size: Some(Vec2::new(36.0, 30.0)), ..default() },
        transform: Transform::from_xyz(0.0, -2.0, 0.15), ..default()
    }, YetiPart));
    // Arms
    for (x, rot) in [(-34.0, 0.3), (34.0, -0.3)] {
        parent.spawn((SpriteBundle {
            sprite: Sprite { color: Color::srgb(0.88, 0.86, 0.84), custom_size: Some(Vec2::new(16.0, 36.0)), ..default() },
            transform: Transform::from_xyz(x, 8.0, 0.2).with_rotation(Quat::from_rotation_z(rot)), ..default()
        }, YetiPart));
    }
    // Claws
    for x in [-38.0, 38.0] {
        parent.spawn((SpriteBundle {
            sprite: Sprite { color: Color::srgb(0.65, 0.62, 0.58), custom_size: Some(Vec2::new(14.0, 12.0)), ..default() },
            transform: Transform::from_xyz(x, 24.0, 0.25), ..default()
        }, YetiPart));
    }
    // Head
    parent.spawn((SpriteBundle {
        sprite: Sprite { color: Color::srgb(0.9, 0.88, 0.86), custom_size: Some(Vec2::new(36.0, 32.0)), ..default() },
        transform: Transform::from_xyz(0.0, 38.0, 0.3), ..default()
    }, YetiPart));
    // Face
    parent.spawn((SpriteBundle {
        sprite: Sprite { color: Color::srgb(0.55, 0.6, 0.7), custom_size: Some(Vec2::new(26.0, 18.0)), ..default() },
        transform: Transform::from_xyz(0.0, 36.0, 0.35), ..default()
    }, YetiPart));
    // Eyes
    for x in [-7.0, 7.0] {
        parent.spawn((SpriteBundle {
            sprite: Sprite { color: Color::srgb(1.0, 0.95, 0.9), custom_size: Some(Vec2::new(8.0, 6.0)), ..default() },
            transform: Transform::from_xyz(x, 40.0, 0.4), ..default()
        }, YetiPart));
        parent.spawn((SpriteBundle {
            sprite: Sprite { color: Color::srgb(0.9, 0.15, 0.1), custom_size: Some(Vec2::new(4.0, 5.0)), ..default() },
            transform: Transform::from_xyz(x, 40.0, 0.45), ..default()
        }, YetiPart));
    }
    // Brow
    parent.spawn((SpriteBundle {
        sprite: Sprite { color: Color::srgb(0.45, 0.48, 0.55), custom_size: Some(Vec2::new(28.0, 4.0)), ..default() },
        transform: Transform::from_xyz(0.0, 44.0, 0.5), ..default()
    }, YetiPart));
    // Mouth
    parent.spawn((SpriteBundle {
        sprite: Sprite { color: Color::srgb(0.2, 0.1, 0.1), custom_size: Some(Vec2::new(16.0, 6.0)), ..default() },
        transform: Transform::from_xyz(0.0, 32.0, 0.4), ..default()
    }, YetiPart));
    // Teeth
    for x in [-4.0, 0.0, 4.0] {
        parent.spawn((SpriteBundle {
            sprite: Sprite { color: Color::srgb(0.95, 0.95, 0.85), custom_size: Some(Vec2::new(3.0, 4.0)), ..default() },
            transform: Transform::from_xyz(x, 33.0, 0.45), ..default()
        }, YetiPart));
    }
    // Horns
    for (x, h) in [(-8.0, 10.0), (0.0, 14.0), (8.0, 10.0)] {
        parent.spawn((SpriteBundle {
            sprite: Sprite { color: Color::srgb(0.85, 0.83, 0.8), custom_size: Some(Vec2::new(8.0, h)), ..default() },
            transform: Transform::from_xyz(x, 52.0 + h * 0.2, 0.25), ..default()
        }, YetiPart));
    }
}
