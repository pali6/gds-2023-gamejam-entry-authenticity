use bevy::prelude::*;
use crate::in_game::animation::components::*;

use super::{click::ChickenClickEvent, components::Chicken};

pub fn click_kill(
    mut commands: Commands,
    mut events: EventReader<ChickenClickEvent>,
    mut chickens: Query<(&mut Chicken, &mut Transform)>,
    asset_server: Res<AssetServer>
) {
    for event in events.read() {
        if event.mouse_button.just_released(MouseButton::Left) {
            let entity = event.chicken;
            if let Ok((chicken, transform)) = chickens.get_mut(entity) {
                println!("Killing chicken {}", chicken.name);
                commands.entity(entity).despawn_recursive();
                // TODO: this is a placeholder, it will need to be something more sophisticated

                let pos = transform.translation;
                commands.spawn((
                    SpriteBundle{
                        texture: asset_server.load("sprites/chicken-dead.png"),
                        transform: Transform::from_xyz(pos.x, pos.y, pos.z),
                        ..default()
                    },
                    FadeAwayTween::new(2.0, EasingFunction::Smooth, true)
                ));
            }
        }
    }
}