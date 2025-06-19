// src/events/combat_event.rs
use bevy::prelude::*;
use crate::components::*;

#[derive(Event)]
pub struct CombatEvent {
    pub attacker: Entity,
    pub target: Entity,
    pub weapon: Option<Item>,
    pub damage: u32,
}

pub fn combat_event_handler(
    mut event_reader: EventReader<CombatEvent>,
    mut query: Query<&mut Health>,
) {
    for event in event_reader.iter() {
        if let Ok(mut target_health) = query.get_mut(event.target) {
            target_health.current = target_health.current.saturating_sub(event.damage);
            println!("{} takes {} damage! Health remaining: {}", 
                event.target.index(), 
                event.damage, 
                target_health.current
            );
        }
    }
}