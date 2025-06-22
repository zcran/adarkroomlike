// src/systems/world_render.rs
use bevy::prelude::*;
use crate::components::world::{WorldConfig, WorldState, TileType};

// 渲染地图
pub fn render_world(
    world_config: Res<WorldConfig>,
    world_state: Res<WorldState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // 清除现有的地图实体
    commands
        .query_filtered::<Entity, With<MapTile>>()
        .for_each(|entity| commands.despawn(entity));
    
    let size = (world_config.radius * 2) as usize;
    let tile_size = 32.0; // 每个瓦片的像素大小
    
    // 创建地图容器
    let mut map_container = commands.spawn(NodeBundle {
        style: Style {
            size: Size::new(Val::Px(tile_size * (size as f32)), Val::Px(tile_size * (size as f32))),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        background_color: Color::rgb(0.1, 0.1, 0.1).into(),
        ..default()
    });
    
    // 渲染地图瓦片
    for y in 0..size {
        for x in 0..size {
            let tile_type = world_state.map[x][y];
            let is_visible = world_state.mask[x][y];
            
            if is_visible {
                let color = match tile_type {
                    TileType::Village => Color::rgb(0.2, 0.6, 0.2),
                    TileType::IronMine => Color::rgb(0.5, 0.5, 0.5),
                    TileType::CoalMine => Color::rgb(0.1, 0.1, 0.1),
                    TileType::SulphurMine => Color::rgb(0.8, 0.8, 0.2),
                    TileType::Forest => Color::rgb(0.1, 0.5, 0.1),
                    TileType::Field => Color::rgb(0.4, 0.8, 0.4),
                    TileType::Barrens => Color::rgb(0.8, 0.6, 0.2),
                    TileType::Road => Color::rgb(0.3, 0.3, 0.3),
                    TileType::House => Color::rgb(0.6, 0.4, 0.2),
                    TileType::Cave => Color::rgb(0.3, 0.3, 0.5),
                    TileType::Town => Color::rgb(0.5, 0.5, 0.5),
                    TileType::City => Color::rgb(0.4, 0.4, 0.4),
                    TileType::Outpost => Color::rgb(0.6, 0.3, 0.3),
                    TileType::Ship => Color::rgb(0.2, 0.2, 0.8),
                    TileType::Borehole => Color::rgb(0.8, 0.2, 0.8),
                    TileType::Battlefield => Color::rgb(0.8, 0.2, 0.2),
                    TileType::Swamp => Color::rgb(0.1, 0.5, 0.4),
                    TileType::Cache => Color::rgb(0.6, 0.6, 0.2),
                };
                
                map_container.with_children(|parent| {
                    parent.spawn(NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            left: Val::Px(tile_size * x as f32),
                            top: Val::Px(tile_size * y as f32),
                            size: Size::new(Val::Px(tile_size), Val::Px(tile_size)),
                            ..default()
                        },
                        background_color: color.into(),
                        ..default()
                    }).insert(MapTile);
                });
            }
        }
    }
    
    // 渲染玩家位置
    let player_x = world_state.cur_pos.x as usize;
    let player_y = world_state.cur_pos.y as usize;
    if world_state.mask[player_x][player_y] {
        map_container.with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(tile_size * player_x as f32),
                    top: Val::Px(tile_size * player_y as f32),
                    size: Size::new(Val::Px(tile_size), Val::Px(tile_size)),
                    ..default()
                },
                background_color: Color::RED.into(),
                ..default()
            }).insert(PlayerMarker);
        });
    }
}

// 地图瓦片组件
#[derive(Component)]
struct MapTile;

// 玩家标记组件
#[derive(Component)]
struct PlayerMarker;