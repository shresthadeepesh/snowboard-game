use std::fmt::Write;
use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::resources::GameData;
use crate::state::GameState;

pub struct ScoringPlugin;

impl Plugin for ScoringPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_score, update_hud).run_if(in_state(GameState::Playing)),
        );
    }
}

fn update_score(time: Res<Time>, mut game_data: ResMut<GameData>, player_query: Query<&Player>) {
    if player_query.get_single().map_or(true, |p| !p.intro_done) {
        return;
    }

    let speed_mult = game_data.speed_multiplier();
    let effective_speed = game_data.scroll_speed * speed_mult;
    let multiplier = game_data.score_multiplier;
    game_data.distance += effective_speed * time.delta_seconds();
    game_data.score = game_data.distance / 10.0 * multiplier;

    if game_data.scroll_speed < MAX_SCROLL_SPEED {
        game_data.scroll_speed += SPEED_INCREASE_RATE * time.delta_seconds();
    }
}

fn update_hud(
    game_data: Res<GameData>,
    mut score_query: Query<&mut Text, (With<ScoreText>, Without<SpeedText>, Without<ComboText>, Without<CoinCountText>)>,
    mut speed_query: Query<&mut Text, (With<SpeedText>, Without<ScoreText>, Without<ComboText>, Without<CoinCountText>)>,
    mut combo_query: Query<&mut Text, (With<ComboText>, Without<ScoreText>, Without<SpeedText>, Without<CoinCountText>)>,
    mut coin_query: Query<&mut Text, (With<CoinCountText>, Without<ScoreText>, Without<SpeedText>, Without<ComboText>)>,
) {
    if !game_data.is_changed() {
        return;
    }

    for mut text in &mut score_query {
        let s = &mut text.sections[0].value;
        s.clear();
        let _ = write!(s, "Score: {}", game_data.score.round() as u32);
    }

    let speed_mult = game_data.speed_multiplier();
    let speed_pct = ((game_data.scroll_speed * speed_mult / BASE_SCROLL_SPEED) * 100.0) as u32;
    for mut text in &mut speed_query {
        let s = &mut text.sections[0].value;
        s.clear();
        if game_data.stunned_timer > 0.0 {
            let _ = write!(s, "Speed: {}% SLOWED", speed_pct);
            text.sections[0].style.color = Color::srgb(1.0, 0.3, 0.2);
        } else {
            let _ = write!(s, "Speed: {}%", speed_pct);
            text.sections[0].style.color = Color::srgb(0.15, 0.15, 0.25);
        }
    }

    for mut text in &mut combo_query {
        if game_data.combo_count > 2 {
            let s = &mut text.sections[0].value;
            s.clear();
            let _ = write!(s, "x{} COMBO", game_data.combo_count);
            let pulse = (game_data.combo_count as f32 * 0.1).min(1.0);
            text.sections[0].style.color = Color::srgb(
                0.2 + pulse * 0.8,
                1.0 - pulse * 0.5,
                0.5 - pulse * 0.3,
            );
        } else {
            text.sections[0].value.clear();
        }
    }

    for mut text in &mut coin_query {
        if game_data.coins_collected > 0 {
            let s = &mut text.sections[0].value;
            s.clear();
            let _ = write!(s, "Coins: {}", game_data.coins_collected);
        } else {
            text.sections[0].value.clear();
        }
    }
}
