use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::effects::collision_particles::spawn_collision_burst;
use crate::resources::{GameData, SfxHandles};
use crate::state::GameState;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (check_collisions, update_stun).run_if(in_state(GameState::Playing)),
        );
    }
}

fn check_collisions(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    obstacle_query: Query<(&Transform, &Obstacle)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut game_data: ResMut<GameData>,
    existing_overlay: Query<Entity, With<StunnedOverlay>>,
    shield_query: Query<Entity, With<ShieldBubble>>,
    sfx: Res<SfxHandles>,
) {
    if game_data.hit_cooldown > 0.0 {
        return;
    }
    // Speed boost = invincible
    if game_data.speed_boost_timer > 0.0 {
        return;
    }

    let Ok(player_tf) = player_query.get_single() else { return };
    let player_pos = player_tf.translation.truncate();

    for (transform, obstacle) in &obstacle_query {
        let obstacle_pos = transform.translation.truncate();
        let distance = player_pos.distance(obstacle_pos);

        if distance < PLAYER_HITBOX_HALF + obstacle.hitbox_radius {
            // Hit sound
            commands.spawn(AudioBundle {
                source: sfx.hit.clone(),
                settings: PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Despawn,
                    volume: bevy::audio::Volume::new(0.5),
                    ..default()
                },
            });

            // Collision particles
            let midpoint = (player_pos + obstacle_pos) / 2.0;
            spawn_collision_burst(&mut commands, midpoint);

            // Screen shake
            game_data.shake_intensity = SHAKE_OBSTACLE_INTENSITY;
            game_data.shake_timer = SHAKE_OBSTACLE_DURATION;

            // Knockback
            if distance > 0.1 {
                game_data.knockback_dir = (player_pos - obstacle_pos).normalize();
                game_data.knockback_timer = KNOCKBACK_DURATION;
            }

            // Reset combo
            game_data.combo_count = 0;
            game_data.combo_timer = 0.0;
            game_data.score_multiplier = 1.0;

            // Shield absorbs the hit
            if game_data.shield_timer > 0.0 {
                game_data.shield_timer = 0.0;
                game_data.hit_cooldown = HIT_COOLDOWN;
                for entity in &shield_query {
                    if let Some(mut e) = commands.get_entity(entity) { e.despawn(); }
                }
                return;
            }

            if game_data.stunned_timer > 0.0 {
                // Second hit while stunned — game over
                game_data.yeti_lunge_pending = 1000.0;
                let final_score = game_data.score as u32;
                if final_score > game_data.high_score {
                    game_data.high_score = final_score;
                }
                game_data.death_timer = DEATH_FREEZE_DURATION;
                next_state.set(GameState::Dying);
                return;
            }

            // First hit — stun
            game_data.stunned_timer = STUN_DURATION;
            game_data.hit_cooldown = HIT_COOLDOWN;
            game_data.yeti_lunge_pending = YETI_HIT_LUNGE;

            if existing_overlay.is_empty() {
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgba(0.9, 0.1, 0.05, 0.35),
                            custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT)),
                            ..default()
                        },
                        transform: Transform::from_xyz(0.0, 0.0, 80.0),
                        ..default()
                    },
                    StunnedOverlay,
                    InGame,
                ));
                commands.spawn((
                    Text2dBundle {
                        text: Text::from_section(
                            "SLOWED!",
                            TextStyle {
                                font_size: 48.0,
                                color: Color::srgb(1.0, 0.3, 0.2),
                                ..default()
                            },
                        )
                        .with_justify(JustifyText::Center),
                        transform: Transform::from_xyz(0.0, 0.0, 81.0),
                        ..default()
                    },
                    StunnedText,
                    InGame,
                ));
            }
            return;
        }
    }
}

fn update_stun(
    mut commands: Commands,
    time: Res<Time>,
    mut game_data: ResMut<GameData>,
    mut overlay_query: Query<(Entity, &mut Sprite), With<StunnedOverlay>>,
    stun_text_query: Query<Entity, With<StunnedText>>,
) {
    let dt = time.delta_seconds();

    if game_data.hit_cooldown > 0.0 {
        game_data.hit_cooldown = (game_data.hit_cooldown - dt).max(0.0);
    }

    if game_data.stunned_timer <= 0.0 {
        return;
    }

    game_data.stunned_timer = (game_data.stunned_timer - dt).max(0.0);

    let stun_pct = game_data.stunned_timer / STUN_DURATION;
    for (_entity, mut sprite) in &mut overlay_query {
        let pulse = (time.elapsed_seconds() * 6.0).sin() * 0.5 + 0.5;
        sprite.color = Color::srgba(0.9, 0.1, 0.05, stun_pct * 0.3 * (0.5 + pulse * 0.5));
    }

    if game_data.stunned_timer <= 0.0 {
        for (entity, _) in &overlay_query {
            if let Some(mut e) = commands.get_entity(entity) { e.despawn(); }
        }
        for entity in &stun_text_query {
            if let Some(mut e) = commands.get_entity(entity) { e.despawn(); }
        }
    }
}
