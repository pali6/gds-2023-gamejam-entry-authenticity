use bevy::prelude::*;
use crate::{in_game::animation::{components::*, resources::AnimationResource}, utilities::play_sfx, timed_sounds::{play_sound_in, TimedSounds}};

use super::{click::ChickenClickEvent, components::Chicken};

pub fn click_kill(
    mut commands: Commands,
    mut events: EventReader<ChickenClickEvent>,
    mut chickens: Query<(&mut Chicken, &mut Transform)>,
    mut timed_sounds: ResMut<TimedSounds>,
    asset_server: Res<AssetServer>,
    animation_resource: Res<AnimationResource>
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
                let smoke_period: f32 = 0.08;
                let smoke_indeces = AnimationResource::SMOKE_INDICES;
                let smoke_fade = smoke_period * smoke_indeces.len() as f32;
                let mut smoke_transform = Transform::from_xyz(pos.x, pos.y, 10.0);
                smoke_transform.scale = Vec3::new(2.0, 2.0, 2.0);

                if chicken.is_fox {
                    commands.spawn((
                        SpriteSheetBundle {
                            texture_atlas: animation_resource.smoke_atlas.clone(),
                            sprite: TextureAtlasSprite::new(AnimationResource::SMOKE_INDICES[0]),
                            transform: smoke_transform,
                            ..default()
                        },
                        Animation::new(smoke_period, AnimationResource::SMOKE_INDICES, false),
                        FadeAwayTween::new(
                            smoke_fade,
                            EasingFunction::Smooth,
                            true
                        )
                    ));

                    commands.spawn((
                        SpriteBundle{
                            texture: asset_server.load("sprites/fox-dead-pali-black.png"),
                            transform: Transform::from_xyz(pos.x, pos.y, pos.z),
                            ..default()
                        },
                        FadeAwayTween::new(smoke_fade * 4.0, EasingFunction::Smooth, true)
                    ));
                    
                } else {
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
}