use bevy::prelude::*;
use crate::{in_game::animation::components::*, utilities::play_sfx, timed_sounds::{play_sound_in, TimedSounds}};

use super::{click::ChickenClickEvent, components::Chicken};

pub fn click_kill(
    mut commands: Commands,
    mut events: EventReader<ChickenClickEvent>,
    mut chickens: Query<(&mut Chicken, &mut Transform)>,
    mut timed_sounds: ResMut<TimedSounds>,
    asset_server: Res<AssetServer>
) {
    for event in events.read() {
        if event.mouse_button.just_released(MouseButton::Left) {
            let entity = event.chicken;
            if let Ok((chicken, transform)) = chickens.get_mut(entity) {
                println!("Killing chicken {}", chicken.name);
                play_sfx("sounds/shoot.ogg".to_string(), &mut commands, &asset_server, 1.0);
                let is_fox = chicken.is_fox;
                play_sound_in(1.0, if is_fox { "sounds/fox_death.ogg" } else { "sounds/death_bwok.ogg" }, 1.0, &mut timed_sounds);
                commands.entity(entity).despawn_recursive();

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