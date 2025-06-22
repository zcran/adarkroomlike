// src/scenes/world.rs
use bevy::prelude::*;
use crate::app_state::AppState;
use crate::components::world::{WorldConfig, WorldState};
use crate::systems::{world_gen, movement, world_render};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::World)
                .with_system(world_gen::setup_world)
                .with_system(init_world_state),
        )
        .add_system_set(
            SystemSet::on_update(AppState::World)
                .with_system(movement::player_movement)
                .with_system(world_render::render_world),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::World)
                .with_system(cleanup_world),
        );
    }
}

// 初始化世界状态
fn init_world_state(mut world_state: ResMut<WorldState>, config: Res<WorldConfig>) {
    world_state.water = config.base_water;
    world_state.health = config.base_health;
    world_state.food_move = 0;
    world_state.water_move = 0;
    world_state.used_outposts.clear();
    world_state.cur_pos = config.village_pos;
    world_state.dead = false;
}

// 清理世界状态
fn cleanup_world(mut world_state: ResMut<WorldState>) {
    world_state.state = None;
}