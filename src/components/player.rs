// src/components/player.rs
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub health: u32,
    pub stamina: u32,
    pub inventory: Vec<Item>,
}

#[derive(Component)]
pub struct Item {
    pub name: String,
    pub item_type: ItemType,
    pub description: String,
    pub quantity: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemType {
    Resource,
    Tool,
    Weapon,
    Consumable,
}