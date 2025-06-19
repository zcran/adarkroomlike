// src/app_state.rs
use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Hash, Debug, States)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    Paused,
    GameOver,
}