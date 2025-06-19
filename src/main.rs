// src/main.rs
mod app_state; //声明app_state模块
mod scenes;
mod systems;

use bevy::prelude::*;
use crate::app_state::AppState;
use crate::scenes::*;
use crate::systems::*;

fn main() {
    App::new()
       .add_plugins(DefaultPlugins.set(WindowPlugin {
           primary_window: Some(Window {
                title: "A Dark Room - Bevy Edition".to_string(),
                ..Default::default()
           }),
           ..Default::default()
       }))
    //    .add_state::<AppState>()
    //    .add_plugin(MainMenuPlugin)
    //    .add_plugin(VillagePlugin)
    //    .add_plugin(OutsidePlugin)
    //    .add_plugin(ShipPlugin)
    //    .add_plugin(SpacePlugin)
       .add_systems(Startup,setup)
       .run();
}

fn setup(mut commands: Commands) {
    // 初始化游戏全局设置
}