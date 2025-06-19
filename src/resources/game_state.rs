// src/resources/game_state.rs
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct GameState {
    pub resources: HashMap<String, u32>,
    pub buildings: HashMap<String, u32>,
    pub unlocked_items: Vec<String>,
    pub player_health: u32,
    pub days_passed: u32,
}

#[derive(Resource)]
pub struct WorldConfig {
    pub map_size: UVec2,
    pub tile_types: Vec<TileType>,
    // 其他世界配置参数
}