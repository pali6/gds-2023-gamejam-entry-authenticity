use bevy::prelude::*;
use crate::{in_game::{animation::{components::*, resources::AnimationResource}, timed_fox_death::{TimedFoxDeath, queue_timed_fox}, inworld_object::InWorldObject, delayed_death_count::{queue_delayed_death_count, DelayedDeathCount, DeathType}}, utilities::play_sfx, timed_sounds::{play_sound_in, TimedSounds}};

use super::{click::ChickenClickEvent, components::Chicken};

#[derive(Component)]
pub struct DeadChicken;

pub fn click_kill(
    mut commands: Commands,
    mut events: EventReader<ChickenClickEvent>,
    mut chickens: Query<(&mut Chicken, &mut Transform)>,
    mut timed_sounds: ResMut<TimedSounds>,
    asset_server: Res<AssetServer>,
    animation_resource: Res<AnimationResource>,
    mut queued_timed_fox: ResMut<TimedFoxDeath>,
    mut queued_death_count: ResMut<DelayedDeathCount>
) {
    for event in events.read() {
        if event.mouse_button.just_released(MouseButton::Left) {
            let entity = event.chicken;
            if let Ok((chicken, transform)) = chickens.get_mut(entity) {
                println!("Killing chicken {}", chicken.name);
                play_sfx("sounds/shoot.ogg".to_string(), &mut commands, &asset_server, 1.0);
                play_sound_in(0.7, if chicken.is_fox { "sounds/fox_death.ogg" } else { "sounds/death_bwok.ogg" }, 1.0, &mut timed_sounds);
                commands.entity(entity).despawn_recursive();

                let pos = transform.translation;
                let smoke_period: f32 = 1.3 / 9.0;
                let mut smoke_transform = Transform::from_xyz(pos.x, pos.y, 10.0);
                smoke_transform.scale = Vec3::new(2.0, 2.0, 2.0);

                commands.spawn((
                    SpriteSheetBundle {
                        texture_atlas: animation_resource.smoke_atlas.clone(),
                        sprite: TextureAtlasSprite::new(AnimationResource::SMOKE_INDICES[0]),
                        transform: smoke_transform,
                        ..default()
                    },
                    Animation::new(smoke_period, AnimationResource::SMOKE_INDICES, false),
                    FadeAwayTween::new(
                        1.0,
                        0.3,
                        EasingFunction::Smooth,
                        true
                    )
                ));

                let entity = commands.spawn((
                    SpriteBundle{
                        texture: asset_server.load("sprites/chicken-dead.png"),
                        transform: Transform::from_xyz(pos.x, pos.y, pos.z),
                        ..default()
                    },
                    DeadChicken,
                    InWorldObject,
                )).id();

                if chicken.is_fox {
                    queue_timed_fox(0.4, entity, &mut queued_timed_fox);
                }

                queue_delayed_death_count(1.0, if chicken.is_fox { DeathType::Fox } else { DeathType::Chicken }, &mut queued_death_count);
            }
        }
    }
}