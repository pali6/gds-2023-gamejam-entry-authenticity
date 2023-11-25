use bevy::prelude::*;

use crate::{in_game::{animation::{resources::AnimationResource, components::*}, chicken::components::Chicken}, world::WorldParams};

use super::components::*;

pub fn update_chicken_behaviours(
    mut chicken_query: Query<(&mut Behavior, &Chicken, &mut Transform, &Children)>,
    mut animation_query: Query<&mut Animation>,
    time: Res<Time>,
    animation_resource: Res<AnimationResource>,
    world_params: Res<WorldParams>
) {
    for (mut behavior, chicken, mut transform, children) in chicken_query.iter_mut() {
        match behavior.state {
            BehaviorState::Moving => {
                behavior.update_movement(&mut transform, chicken, &world_params);
                for &child in children.iter() {
                    if let Ok(mut anim) = animation_query.get_mut(child) {
                        anim.set_state(AnimState::Running);
                    }
                }
            }

            BehaviorState::Waiting => {
                behavior.update_waiting(&time, &world_params);
                for &child in children.iter() {
                    if let Ok(mut anim) = animation_query.get_mut(child) {
                        //let state = [AnimState::Idle, AnimState::Chilling1][rand::random::<usize>() % 2];
                        anim.set_state(AnimState::Chilling1);
                    }
                }
            }

            BehaviorState::Eating => {
                behavior.update_eating(&time, &world_params);
                for &child in children.iter() {
                    if let Ok(mut anim) = animation_query.get_mut(child) {
                        anim.set_state(AnimState::Eating);
                    }
                }
            }

            BehaviorState::Hiding => {
                behavior.update_hidnig(&time, &world_params);
                for &child in children.iter() {
                    if let Ok(mut anim) = animation_query.get_mut(child) {
                        anim.set_state(AnimState::Chilling2);
                    }
                }
            }

            _ => {}
        };
    }
}