mod movement;
mod spawn;

use bevy::prelude::*;
use crate::components::InGame;
use crate::state::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (movement::player_movement, movement::pause_input)
                .run_if(in_state(GameState::Playing)),
        );
    }
}

pub use spawn::spawn_player;
pub use spawn::spawn_background;

/// Cleanup all in-game entities (used by state transitions).
pub fn cleanup_game(
    mut commands: Commands,
    game_data: Res<crate::resources::GameData>,
    query: Query<Entity, With<InGame>>,
) {
    if game_data.is_paused_resume {
        return;
    }
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
