mod menu;
mod pause;
mod game_over;
mod hud;

use bevy::prelude::*;
use crate::components::*;
use crate::state::GameState;
use crate::player::cleanup_game;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            // Menu
            .add_systems(OnEnter(GameState::Menu), (cleanup_game, menu::setup_menu).chain())
            .add_systems(OnExit(GameState::Menu), cleanup::<MenuUI>)
            .add_systems(Update, menu::menu_input.run_if(in_state(GameState::Menu)))
            // Paused
            .add_systems(OnEnter(GameState::Paused), pause::setup_paused)
            .add_systems(OnExit(GameState::Paused), cleanup::<PausedUI>)
            .add_systems(Update, pause::paused_input.run_if(in_state(GameState::Paused)))
            // Game Over
            .add_systems(OnEnter(GameState::GameOver), (cleanup_game, game_over::setup_game_over).chain())
            .add_systems(OnExit(GameState::GameOver), cleanup::<GameOverUI>)
            .add_systems(Update, game_over::game_over_input.run_if(in_state(GameState::GameOver)));
    }
}

fn cleanup<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

pub use hud::spawn_hud;
