pub mod coins;
pub mod powerups;

use bevy::prelude::*;
use crate::state::GameState;

pub struct CollectiblesPlugin;

impl Plugin for CollectiblesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                coins::spawn_coins,
                coins::update_coins,
                powerups::spawn_powerups,
                powerups::collect_powerups,
                powerups::update_powerup_timers,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}
