mod snow;
mod trail;
mod terrain;

use bevy::prelude::*;
use crate::state::GameState;

pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                snow::spawn_snow,
                snow::update_snow,
                trail::spawn_trail,
                trail::update_trails,
                terrain::spawn_terrain_marks,
                terrain::update_terrain_marks,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}
