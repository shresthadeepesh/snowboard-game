use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    Paused,
    /// Brief freeze after death — shake plays, world visible, then transitions to GameOver.
    Dying,
    GameOver,
}
