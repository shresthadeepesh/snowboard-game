mod spawner;
mod scroll;
mod tree;
mod rock;
mod animals;

use bevy::prelude::*;
use crate::state::GameState;

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawner::spawn_obstacles, scroll::scroll_obstacles, scroll::animate_hopping)
                .run_if(in_state(GameState::Playing)),
        );
    }
}
