// src/systems/world_gen.rs
use bevy::prelude::*;
use crate::components::world::{
    WorldConfig, WorldState, TileType, Landmark, Player,
};
use std::collections::HashMap;
use rand::Rng;

// 初始化世界配置和状态
pub fn setup_world(mut commands: Commands) {
    let mut config = WorldConfig::default();
    
    // 设置基础配置
    config.radius = 30;
    config.village_pos = UVec2::new(30, 30);
    config.stickiness = 0.5;
    config.light_radius = 2;
    config.fight_chance = 0.20;
    config.base_health = 10;
    config.base_hit_chance = 0.8;
    config.meat_heal = 8;
    config.meds_heal = 20;
    config.fight_delay = 3;
    
    // 设置瓦片概率
    config.tile_probs.insert(TileType::Forest, 0.15);
    config.tile_probs.insert(TileType::Field, 0.35);
    config.tile_probs.insert(TileType::Barrens, 0.5);
    
    // 设置地标
    config.landmarks.insert(
        TileType::Outpost,
        Landmark {
            num: 0,
            min_radius: 0,
            max_radius: 0,
            scene: "outpost".to_string(),
            label: "An Outpost".to_string(),
        },
    );
    config.landmarks.insert(
        TileType::IronMine,
        Landmark {
            num: 1,
            min_radius: 5,
            max_radius: 5,
            scene: "ironmine".to_string(),
            label: "Iron Mine".to_string(),
        },
    );
    config.landmarks.insert(
        TileType::CoalMine,
        Landmark {
            num: 1,
            min_radius: 10,
            max_radius: 10,
            scene: "coalmine".to_string(),
            label: "Coal Mine".to_string(),
        },
    );
    config.landmarks.insert(
        TileType::SulphurMine,
        Landmark {
            num: 1,
            min_radius: 20,
            max_radius: 20,
            scene: "sulphurmine".to_string(),
            label: "Sulphur Mine".to_string(),
        },
    );
    config.landmarks.insert(
        TileType::House,
        Landmark {
            num: 10,
            min_radius: 0,
            max_radius: (WorldConfig::radius * 1.5) as u32,
            scene: "house".to_string(),
            label: "An Old House".to_string(),
        },
    );
    config.landmarks.insert(
        TileType::Cave,
        Landmark {
            num: 5,
            min_radius: 3,
            max_radius: 10,
            scene: "cave".to_string(),
            label: "A Damp Cave".to_string(),
        },
    );
    config.landmarks.insert(
        TileType::Town,
        Landmark {
            num: 10,
            min_radius: 10,
            max_radius: 20,
            scene: "town".to_string(),
            label: "An Abandoned Town".to_string(),
        },
    );
    config.landmarks.insert(
        TileType::City,
        Landmark {
            num: 20,
            min_radius: 20,
            max_radius: (WorldConfig::radius * 1.5) as u32,
            scene: "city".to_string(),
            label: "A Ruined City".to_string(),
        },
    );
    config.landmarks.insert(
        TileType::Ship,
        Landmark {
            num: 1,
            min_radius: 28,
            max_radius: 28,
            scene: "ship".to_string(),
            label: "A Crashed Starship".to_string(),
        },
    );
    config.landmarks.insert(
        TileType::Borehole,
        Landmark {
            num: 10,
            min_radius: 15,
            max_radius: (WorldConfig::radius * 1.5) as u32,
            scene: "borehole".to_string(),
            label: "A Borehole".to_string(),
        },
    );
    config.landmarks.insert(
        TileType::Battlefield,
        Landmark {
            num: 5,
            min_radius: 18,
            max_radius: (WorldConfig::radius * 1.5) as u32,
            scene: "battlefield".to_string(),
            label: "A Battlefield".to_string(),
        },
    );
    config.landmarks.insert(
        TileType::Swamp,
        Landmark {
            num: 1,
            min_radius: 15,
            max_radius: (WorldConfig::radius * 1.5) as u32,
            scene: "swamp".to_string(),
            label: "A Murky Swamp".to_string(),
        },
    );
    
    // 设置武器
    // enum Damage {
    //     Fixed(u32),
    //     StatusEffect(String),
    // }
    config.weapons.insert(
        "fists".to_string(),
        Weapon {
            verb: "punch".to_string(),
            weapon_type: "unarmed".to_string(),
            damage: Damage::Fixed(1),
            cooldown: 2,
            cost: None,
        },
    );
    config.weapons.insert(
        "bone spear".to_string(),
        Weapon {
            verb: "stab".to_string(),
            weapon_type: "melee".to_string(),
            damage: Damage::Fixed(2),
            cooldown: 2,
            cost: None,
        },
    );
    config.weapons.insert(
        "iron sword".to_string(),
        Weapon {
            verb: "swing".to_string(),
            weapon_type: "melee".to_string(),
            damage: Damage::Fixed(4),
            cooldown: 2,
            cost: None,
        },
    );
    config.weapons.insert(
        "steel sword".to_string(),
        Weapon {
            verb: "slash".to_string(),
            weapon_type: "melee".to_string(),
            damage: Damage::Fixed(6),
            cooldown: 2,
            cost: None,
        },
    );
    config.weapons.insert(
        "bayonet".to_string(),
        Weapon {
            verb: "thrust".to_string(),
            weapon_type: "melee".to_string(),
            damage: Damage::Fixed(8),
            cooldown: 2,
            cost: None,
        },
    );
    config.weapons.insert(
        "rifle".to_string(),
        Weapon {
            verb: "shoot".to_string(),
            weapon_type: "ranged".to_string(),
            damage: Damage::Fixed(5),
            cooldown: 1,
            cost: Some(vec![("bullets".to_string(), 1)]),
        },
    );
    config.weapons.insert(
        "laser rifle".to_string(),
        Weapon {
            verb: "blast".to_string(),
            weapon_type: "ranged".to_string(),
            damage: Damage::Fixed(8),
            cooldown: 1,
            cost: Some(vec![("energy cell".to_string(), 1)]),
        },
    );
    config.weapons.insert(
        "grenade".to_string(),
        Weapon {
            verb: "lob".to_string(),
            weapon_type: "ranged".to_string(),
            damage: Damage::Fixed(15),
            cooldown: 5,
            cost: Some(vec![("grenade".to_string(), 1)]),
        },
    );
    config.weapons.insert(
        "bolas".to_string(),
        Weapon {
            verb: "tangle".to_string(),
            weapon_type: "ranged".to_string(),
            damage: Damage::StatusEffect("stun".to_string()),
            cooldown: 15,
            cost: Some(vec![("bolas".to_string(), 1)]),
        },
    );
    
    // 生成地图
    let mut state = WorldState::default();
    state.map = generate_map(&config);
    state.mask = new_mask(&config);
    
    // 插入资源
    commands.insert_resource(config);
    commands.insert_resource(state);
    
    // 初始化玩家
    let player = Player {
        max_health: config.base_health,
        hit_chance: config.base_hit_chance,
        max_water: config.base_water,
    };
    commands.spawn((player, Transform::default()));
}

// 生成地图
fn generate_map(config: &WorldConfig) -> Vec<Vec<TileType>> {
    let size = (config.radius * 2) as usize;
    let mut map = vec![vec![TileType::Barrens; size]; size];
    let mut rng = rand::thread_rng();
    
    // 设置村庄位置
    let village_x = config.village_pos.x as usize;
    let village_y = config.village_pos.y as usize;
    map[village_x][village_y] = TileType::Village;
    
    // 螺旋生成地图
    for r in 1..=config.radius {
        for t in 0..r * 8 {
            let (mut x, mut y);
            if t < 2 * r {
                x = village_x - r + t;
                y = village_y - r;
            } else if t < 4 * r {
                x = village_x + r;
                y = village_y - (3 * r) + t;
            } else if t < 6 * r {
                x = village_x + (5 * r) - t;
                y = village_y + r;
            } else {
                x = village_x - r;
                y = village_y + (7 * r) - t;
            }
            
            // 确保坐标在范围内
            x = x.clamp(0, size - 1);
            y = y.clamp(0, size - 1);
            
            map[x][y] = choose_tile(x, y, &map, config, &mut rng);
        }
    }
    
    // 放置地标
    place_landmarks(&mut map, config, &mut rng);
    
    map
}

// 选择瓦片类型
fn choose_tile(
    x: usize, 
    y: usize, 
    map: &[Vec<TileType>], 
    config: &WorldConfig, 
    rng: &mut impl Rng,
) -> TileType {
    let size = map.len();
    let mut adjacent = [None; 4];
    
    // 检查相邻瓦片
    if y > 0 {
        adjacent[0] = Some(map[x][y-1]);
    }
    if y < size - 1 {
        adjacent[1] = Some(map[x][y+1]);
    }
    if x < size - 1 {
        adjacent[2] = Some(map[x+1][y]);
    }
    if x > 0 {
        adjacent[3] = Some(map[x-1][y]);
    }
    
    let mut chances = HashMap::new();
    let mut non_sticky = 1.0;
    
    // 计算粘性概率
    for tile in adjacent.iter().flatten() {
        if *tile == TileType::Village {
            return TileType::Forest; // 村庄周围必须是森林
        }
        if let Some(weight) = chances.get_mut(tile) {
            *weight += config.stickiness;
        } else {
            chances.insert(*tile, config.stickiness);
        }
        non_sticky -= config.stickiness;
    }
    
    // 计算非粘性概率
    for (tile_type, prob) in config.tile_probs.iter() {
        if is_terrain(*tile_type) {
            let weight = chances.entry(*tile_type).or_insert(0.0);
            *weight += prob * non_sticky;
        }
    }
    
    // 选择瓦片
    let mut sorted = chances.iter().collect::<Vec<_>>();
    sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    
    let r = rng.gen::<f32>();
    let mut cumulative = 0.0;
    for (tile, prob) in sorted {
        cumulative += prob;
        if r < cumulative {
            return *tile;
        }
    }
    
    TileType::Barrens
}

// 放置地标
fn place_landmarks(map: &mut [Vec<TileType>], config: &WorldConfig, rng: &mut impl Rng) {
    let size = map.len();
    let village_pos = config.village_pos;
    
    for (tile_type, landmark) in config.landmarks.iter() {
        for _ in 0..landmark.num {
            place_landmark(map, tile_type, landmark, village_pos, size, rng);
        }
    }
}

// 放置单个地标
fn place_landmark(
    map: &mut [Vec<TileType>], 
    tile_type: &TileType, 
    landmark: &Landmark, 
    village_pos: UVec2, 
    size: usize, 
    rng: &mut impl Rng,
) {
    let village_x = village_pos.x as usize;
    let village_y = village_pos.y as usize;
    let mut x = village_x;
    let mut y = village_y;
    
    while !is_terrain(map[x][y]) {
        let r = rng.gen_range(landmark.min_radius, landmark.max_radius + 1);
        let x_dist = rng.gen_range(0, r + 1);
        let y_dist = r - x_dist;
        
        if rng.gen::<bool>() {
            x_dist = -x_dist as i32;
        }
        if rng.gen::<bool>() {
            y_dist = -y_dist as i32;
        }
        
        x = (village_x as i32 + x_dist) as usize;
        y = (village_y as i32 + y_dist) as usize;
        
        // 确保坐标在范围内
        x = x.clamp(0, size - 1);
        y = y.clamp(0, size - 1);
    }
    
    map[x][y] = *tile_type;
}

// 判断是否为地形瓦片
fn is_terrain(tile: TileType) -> bool {
    matches!(tile, TileType::Forest | TileType::Field | TileType::Barrens)
}

// 创建新的地图掩码
fn new_mask(config: &WorldConfig) -> Vec<Vec<bool>> {
    let size = (config.radius * 2) as usize;
    let mut mask = vec![vec![false; size]; size];
    light_map(config.village_pos.x as usize, config.village_pos.y as usize, config, &mut mask);
    mask
}

// 更新地图光照
fn light_map(x: usize, y: usize, config: &WorldConfig, mask: &mut [Vec<bool>]) {
    let radius = (config.light_radius as f32 * (if $SM.has_perk('scout') {2.0} else {1.0})).round() as i32;
    uncover_map(x as i32, y as i32, radius, mask);
}

// 揭开地图迷雾
fn uncover_map(x: i32, y: i32, radius: i32, mask: &mut [Vec<bool>]) {
    let size = mask.len() as i32;
    mask[x as usize][y as usize] = true;
    
    for i in -radius..=radius {
        for j in -radius + i.abs()..=radius - i.abs() {
            let nx = x + i;
            let ny = y + j;
            if nx >= 0 && nx < size && ny >= 0 && ny < size {
                mask[nx as usize][ny as usize] = true;
            }
        }
    }
}