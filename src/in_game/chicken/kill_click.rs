use bevy::prelude::*;

use super::{click::ChickenClickEvent, components::Chicken};

pub fn click_kill(
    mut commands: Commands,
    mut events: EventReader<ChickenClickEvent>,
    mut chickens: Query<&mut Chicken>,
) {
    for event in events.read() {
        if event.mouse_button.just_released(MouseButton::Left) {
            let entity = event.chicken;
            if let Ok(chicken) = chickens.get_mut(entity) {
                println!("Killing chicken {}", chicken.name);
                commands.entity(entity).despawn_recursive();
                // TODO: this is a placeholder, it will need to be something more sophisticated
            }
        }
    }
}