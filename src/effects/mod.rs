pub mod shake;
pub mod speed_lines;
pub mod collision_particles;
pub mod near_miss;
pub mod floating_text;
pub mod milestones;

use bevy::prelude::*;
use crate::state::GameState;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app
            // These run always (needed during Dying state)
            .add_systems(Update, (
                shake::apply_screen_shake,
                collision_particles::update_collision_particles,
                floating_text::update_floating_text,
            ))
            .add_systems(
                Update,
                (
                    speed_lines::spawn_speed_lines,
                    speed_lines::update_speed_lines,
                    near_miss::check_near_misses,
                    near_miss::update_combo,
                    milestones::check_milestones,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
