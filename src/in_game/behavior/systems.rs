use bevy::prelude::*;

use crate::{in_game::{animation::{components::*, resources::AnimationResource}, chicken::components::{Chicken, ChickenPart}}, world::WorldParams};

use super::components::*;

pub fn update_chicken_behaviours(
    mut commands: Commands,
    mut chicken_query: Query<(Entity, &mut Behavior, &Chicken, &mut Transform, &Children)>,
    mut chicken_parts: Query<&mut Transform, (With<ChickenPart>, Without<Chicken>)>,
    mut animation_query: Query<&mut Animation>,
    time: Res<Time>,
    world_params: Res<WorldParams>,
    anim_resource: Res<AnimationResource>
) {
    for (entity , mut behavior, chicken, mut transform, children) in chicken_query.iter_mut() {
        let initial_pos = transform.translation.clone();

        match behavior.state {
            BehaviorState::Moving => {
                behavior.update_movement(&mut transform, chicken, &world_params, entity, &mut commands, &anim_resource, &time);
                for &child in children.iter() {
                    if let Ok(mut anim) = animation_query.get_mut(child) {
                        anim.set_state(AnimState::Running);
                    }
                }
            }

            BehaviorState::Waiting => {
                behavior.update_waiting(&time, &world_params, entity, &mut commands, &anim_resource, chicken, &mut transform);
                for &child in children.iter() {
                    if let Ok(mut anim) = animation_query.get_mut(child) {
                        anim.set_state(AnimState::Chilling1);
                    }
                }
            }

            BehaviorState::Eating => {
                behavior.update_eating(&time, &world_params, entity, &mut commands, &anim_resource, chicken, &mut transform);
                for &child in children.iter() {
                    if let Ok(mut anim) = animation_query.get_mut(child) {
                        anim.set_state(AnimState::Eating);
                    }
                }
            }

            BehaviorState::Hiding => {
                behavior.update_hidnig(&time, &world_params, entity, &mut commands, &anim_resource, chicken, &mut transform);
                for &child in children.iter() {
                    if let Ok(mut anim) = animation_query.get_mut(child) {
                        anim.set_state(AnimState::Chilling2);
                    }
                }
            }

            _ => {}
        };

        let move_dir = transform.translation - initial_pos;

        for child in children.iter() {
            if let Ok(mut part_trans) = chicken_parts.get_mut(*child) {
                if move_dir.x > 0.1 {
                    part_trans.scale = Vec3::new(-1.0, 1.0, 1.0);
                } else if move_dir.x < -0.1 {
                    part_trans.scale = Vec3::new(1.0, 1.0, 1.0);
                }
            }
        }
    }
}

pub fn update_speech_bubbles(
    mut commands: Commands,
    mut bubble_query: Query<(Entity, &mut SpeechBubble)>,
    time: Res<Time>
) {
    for (entity, mut bubble) in bubble_query.iter_mut() {
        bubble.destroy_timer.tick(time.delta());

        if bubble.destroy_timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}