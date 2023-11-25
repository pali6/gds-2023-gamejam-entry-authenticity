use bevy::prelude::*;

use crate::{in_game::{animation::{resources::AnimationResource, components::Animation}, chicken::components::Chicken}, world::WorldParams};

use super::components::*;

pub fn update_chicken_behaviours(
    mut chicken_query: Query<(&mut Behavior, &Chicken, &mut Transform, &mut Animation)>,
    time: Res<Time>,
    animation_resource: Res<AnimationResource>,
    world_params: Res<WorldParams>
) {
    for (mut behavior, chicken, mut transform, mut animation) in chicken_query.iter_mut() {
        match behavior.state {
            BehaviorState::Moving => {
                behavior.update_movement(&mut transform, &mut animation, chicken, &world_params);
            }

            BehaviorState::Waiting => {
                behavior.update_waiting(&time, &world_params);
            }

            BehaviorState::Eating => {
                behavior.update_eating(&time, &world_params);
            }

            BehaviorState::Hiding => {
                behavior.update_hidnig(&time, &world_params);
            }

            _ => {}
        };
    }
}