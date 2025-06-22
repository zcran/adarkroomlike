// src/components/world.rs
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use rand::Rng;

// 地图瓦片类型枚举
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TileType {
    Village,
    IronMine,
    CoalMine,
    SulphurMine,
    Forest,
    Field,
    Barrens,
    Road,
    House,
    Cave,
    Town,
    City,
    Outpost,
    Ship,
    Borehole,
    Battlefield,
    Swamp,
    Cache,
}

// 武器结构体
#[derive(Clone, Debug)]
pub struct Weapon {
    pub verb: String,
    pub weapon_type: String,
    pub damage: u32,
    pub cooldown: u32,
    pub cost: Option<HashMap<String, u32>>,
}

// 地标结构体
#[derive(Clone, Debug)]
pub struct Landmark {
    pub num: u32,
    pub min_radius: u32,
    pub max_radius: u32,
    pub scene: String,
    pub label: String,
}

// 世界配置资源
#[derive(Resource, Debug, Default)]
pub struct WorldConfig {
    pub radius: u32,
    pub village_pos: UVec2,
    pub tile_probs: HashMap<TileType, f32>,
    pub landmarks: HashMap<TileType, Landmark>,
    pub stickiness: f32,
    pub light_radius: u32,
    pub base_water: u32,
    pub moves_per_food: u32,
    pub moves_per_water: u32,
    pub death_cooldown: u32,
    pub fight_chance: f32,
    pub base_health: u32,
    pub base_hit_chance: f32,
    pub meat_heal: u32,
    pub meds_heal: u32,
    pub fight_delay: u32,
    pub weapons: HashMap<String, Weapon>,
}

// 世界状态资源
#[derive(Resource, Debug)]
pub struct WorldState {
    pub map: Vec<Vec<TileType>>,
    pub mask: Vec<Vec<bool>>,
    pub cur_pos: UVec2,
    pub water: u32,
    pub health: u32,
    pub outfit: HashMap<String, u32>,
    pub used_outposts: HashSet<String>,
    pub food_move: u32,
    pub water_move: u32,
    pub fight_move: u32,
    pub dead: bool,
}

impl Default for WorldState {
    fn default() -> Self {
        let config = WorldConfig::default();
        Self {
            map: vec![vec![TileType::Barrens; (config.radius * 2) as usize]; (config.radius * 2) as usize],
            mask: vec![vec![false; (config.radius * 2) as usize]; (config.radius * 2) as usize],
            cur_pos: config.village_pos,
            water: config.base_water,
            health: config.base_health,
            outfit: HashMap::new(),
            used_outposts: HashSet::new(),
            food_move: 0,
            water_move: 0,
            fight_move: 0,
            dead: false,
        }
    }
}

// 玩家组件
#[derive(Component, Debug)]
pub struct Player {
    pub max_health: u32,
    pub hit_chance: f32,
    pub max_water: u32,
}