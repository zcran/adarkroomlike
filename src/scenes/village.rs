// src/scenes/village.rs
use bevy::prelude::*;
use crate::app_state::AppState;
use crate::resources::*;
use crate::systems::*;

pub struct VillagePlugin;

impl Plugin for VillagePlugin {
    fn build(&self, app: &mut App) {
        app
           .add_system_set(
                SystemSet::on_enter(AppState::Village)
                   .with_system(setup_village)
            )
           .add_system_set(
                SystemSet::on_update(AppState::Village)
                   .with_system(village_interaction)
                   .with_system(update_village_ui)
            )
           .add_system_set(
                SystemSet::on_exit(AppState::Village)
                   .with_system(cleanup_village)
            );
    }
}

fn setup_village(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 设置村庄场景
}