#![allow(dead_code)]

use bevy::prelude::*;

use super::components::Chicken;
use super::click::ChickenClickEvent;


pub fn debug_chicken_click(
    mut events: EventReader<ChickenClickEvent>,
    query: Query<&mut Chicken>,
) {
    for event in events.read() {
        if let Ok(chicken) = query.get(event.chicken) {
            if event.mouse_button.just_pressed(MouseButton::Left) {
                println!("Chicken clicked: {}", chicken.name);
            }
        }
    }
}