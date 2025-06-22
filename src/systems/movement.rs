// src/systems/movement.rs
use bevy::prelude::*;
use crate::components::world::{
    WorldConfig, WorldState, TileType, Player,
};
use crate::app_state::AppState;
use rand::Rng;

// 玩家移动系统
pub fn player_movement(
    mut commands: Commands,
    mut world_config: ResMut<WorldConfig>,
    mut world_state: ResMut<WorldState>,
    mut player: Query<&mut Player>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    if world_state.dead {
        return;
    }
    
    let mut new_pos = world_state.cur_pos;
    let size = (world_config.radius * 2) as u32;
    let mut moved = false;
    
    // 处理键盘输入
    if keyboard_input.just_pressed(KeyCode::W) && new_pos.y > 0 {
        new_pos.y -= 1;
        moved = true;
    } else if keyboard_input.just_pressed(KeyCode::S) && new_pos.y < size - 1 {
        new_pos.y += 1;
        moved = true;
    } else if keyboard_input.just_pressed(KeyCode::A) && new_pos.x > 0 {
        new_pos.x -= 1;
        moved = true;
    } else if keyboard_input.just_pressed(KeyCode::D) && new_pos.x < size - 1 {
        new_pos.x += 1;
        moved = true;
    }
    
    if moved {
        let old_tile = world_state.map[world_state.cur_pos.x as usize][world_state.cur_pos.y as usize];
        let new_tile = world_state.map[new_pos.x as usize][new_pos.y as usize];
        
        // 叙述移动
        narrate_move(old_tile, new_tile);
        
        // 更新光照
        light_map(new_pos.x as usize, new_pos.y as usize, &world_config, &mut world_state.mask);
        
        // 更新当前位置
        world_state.cur_pos = new_pos;
        
        // 检查是否到达村庄
        if new_tile == TileType::Village {
            go_home(&mut world_state, &mut player.single_mut(), &mut app_state);
            return;
        }
        
        // 使用补给品
        if !use_supplies(&mut world_state, &world_config) {
            world_state.dead = true;
            die(&mut world_state, &mut app_state);
            return;
        }
        
        // 检查战斗
        check_fight(&mut world_state, &world_config);
        
        // 检查危险
        check_danger(&world_state, &world_config);
    }
}

// 叙述移动
fn narrate_move(old_tile: TileType, new_tile: TileType) {
    // 实现叙述逻辑
    println!("Moved from {:?} to {:?}", old_tile, new_tile);
}

// 使用补给品
fn use_supplies(world_state: &mut WorldState, config: &WorldConfig) -> bool {
    world_state.food_move += 1;
    world_state.water_move += 1;
    
    // 食物消耗
    let moves_per_food = config.moves_per_food * (if $SM.has_perk('slow metabolism') {2} else {1});
    if world_state.food_move >= moves_per_food {
        world_state.food_move = 0;
        let mut meat = world_state.outfit.entry("cured meat".to_string()).or_insert(0);
        *meat = *meat.saturating_sub(1);
        
        if *meat == 0 {
            println!("The meat has run out");
        } else if *meat < 0 {
            *meat = 0;
            if !world_state.starvation {
                println!("Starvation sets in");
                world_state.starvation = true;
            } else {
                // 饥饿致死
                return false;
            }
        } else {
            world_state.starvation = false;
            let heal = config.meat_heal * (if $SM.has_perk('gastronome') {2} else {1});
            world_state.health = world_state.health.saturating_add(heal);
            if world_state.health > get_max_health(world_state) {
                world_state.health = get_max_health(world_state);
            }
        }
    }
    
    // 水消耗
    let moves_per_water = config.moves_per_water * (if $SM.has_perk('desert rat') {2} else {1});
    if world_state.water_move >= moves_per_water {
        world_state.water_move = 0;
        world_state.water = world_state.water.saturating_sub(1);
        
        if world_state.water == 0 {
            println!("There is no more water");
        } else if world_state.water < 0 {
            world_state.water = 0;
            if !world_state.thirst {
                println!("The thirst becomes unbearable");
                world_state.thirst = true;
            } else {
                // 口渴致死
                return false;
            }
        } else {
            world_state.thirst = false;
        }
    }
    
    true
}

// 检查战斗
fn check_fight(world_state: &mut WorldState, config: &WorldConfig) {
    world_state.fight_move += 1;
    if world_state.fight_move > config.fight_delay {
        let chance = config.fight_chance * (if $SM.has_perk('stealthy') {0.5} else {1.0});
        if rand::thread_rng().gen::<f32>() < chance {
            world_state.fight_move = 0;
            // 触发战斗事件
            println!("Encountered an enemy!");
        }
    }
}

// 检查危险
fn check_danger(world_state: &WorldState, config: &WorldConfig) {
    let distance = get_distance(&world_state.cur_pos, &config.village_pos);
    
    if !world_state.danger {
        if $SM.get('stores["i armour"]', true) == 0 && distance >= 8 {
            world_state.danger = true;
        } else if $SM.get('stores["s armour"]', true) == 0 && distance >= 18 {
            world_state.danger = true;
        }
    } else {
        if distance < 8 {
            world_state.danger = false;
        } else if distance < 18 && $SM.get('stores["i armour"]', true) > 0 {
            world_state.danger = false;
        }
    }
}

// 回家逻辑
fn go_home(world_state: &mut WorldState, player: &mut Player, app_state: &mut NextState<AppState>) {
    // 保存世界状态
    // 增加建筑
    if world_state.sulphurmine && $SM.get('game.buildings["sulphur mine"]', true) == 0 {
        $SM.add('game.buildings["sulphur mine"]', 1);
    }
    if world_state.ironmine && $SM.get('game.buildings["iron mine"]', true) == 0 {
        $SM.add('game.buildings["iron mine"]', 1);
    }
    if world_state.coalmine && $SM.get('game.buildings["coal mine"]', true) == 0 {
        $SM.add('game.buildings["coal mine"]', 1);
    }
    if world_state.ship && !$SM.get('features.location.spaceShip') {
        // 初始化飞船
    }
    
    // 重置世界状态
    world_state.state = None;
    
    // 保存装备
    for (item, count) in &world_state.outfit {
        $SM.add(&format!("stores[\"{}\"]", item), *count);
        if leave_it_at_home(item) {
            world_state.outfit.insert(item.clone(), 0);
        }
    }
    
    // 切换到路径场景
    app_state.set(AppState::Path);
}

// 判断是否将物品留在家里
fn leave_it_at_home(item: &str) -> bool {
    !["cured meat", "bullets", "energy cell", "charm", "medicine"].contains(&item)
        && World.Weapons.get(item).is_none()
        && Room.Craftables.get(item).is_none()
}

// 计算距离
fn get_distance(pos1: &UVec2, pos2: &UVec2) -> u32 {
    (pos1.x.abs_diff(pos2.x) + pos1.y.abs_diff(pos2.y))
}

// 获取最大生命值
fn get_max_health(world_state: &WorldState) -> u32 {
    if $SM.get('stores["s armour"]', true) > 0 {
        world_config.base_health + 35
    } else if $SM.get('stores["i armour"]', true) > 0 {
        world_config.base_health + 15
    } else if $SM.get('stores["l armour"]', true) > 0 {
        world_config.base_health + 5
    } else {
        world_config.base_health
    }
}

// 玩家死亡逻辑
fn die(world_state: &mut WorldState, app_state: &mut NextState<AppState>) {
    if !world_state.dead {
        world_state.dead = true;
        println!("Player died!");
        
        // 重置世界状态
        world_state.state = None;
        world_state.outfit.clear();
        $SM.remove("outfit");
        
        // 切换到房间场景
        app_state.set(AppState::Room);
    }
}