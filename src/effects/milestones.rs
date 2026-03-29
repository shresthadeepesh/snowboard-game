use bevy::prelude::*;
use crate::constants::*;
use crate::resources::GameData;
use super::floating_text::spawn_floating_text;

pub fn check_milestones(mut commands: Commands, mut game_data: ResMut<GameData>) {
    if game_data.next_milestone_index >= MILESTONE_DISTANCES.len() {
        return;
    }

    let threshold = MILESTONE_DISTANCES[game_data.next_milestone_index];
    if game_data.distance >= threshold {
        let display_dist = (threshold / 10.0) as u32;
        let text = format!("{}m!", display_dist);
        spawn_floating_text(
            &mut commands,
            &text,
            Vec2::new(0.0, 50.0),
            Color::srgb(1.0, 0.85, 0.2),
            36.0,
            MILESTONE_TEXT_DURATION,
            FLOATING_TEXT_RISE_SPEED * 0.5,
        );
        game_data.next_milestone_index += 1;
    }
}
