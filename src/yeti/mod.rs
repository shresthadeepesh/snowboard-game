mod spawn;
mod behavior;

use bevy::prelude::*;
use crate::state::GameState;

pub struct YetiPlugin;

impl Plugin for YetiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn::spawn_yeti, behavior::update_yeti)
                .run_if(in_state(GameState::Playing)),
        );
    }
}
