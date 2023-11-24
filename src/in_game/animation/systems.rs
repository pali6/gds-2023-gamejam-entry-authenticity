use bevy::prelude::*;
use crate::in_game::player::components::Player;

use super::{components::*, resources::AnimationResource};

pub fn load_animation_resources(
    mut animation_resource: ResMut<AnimationResource>,
    asset_server: Res<AssetServer>
) {
    animation_resource.frame_period = 0.8;

    // TODO
    //animation_resource.hen_idle.push(asset_server.load("sprites/idle1.jpg"));
}

pub fn update_animations(
    animation_resource: Res<AnimationResource>,
    mut animation_query: Query<(&mut Animation, &mut Handle<Image>)>,
    time: Res<Time>
) {

    for (mut animation, mut image) in animation_query.iter_mut() {
        animation.timer.tick(time.delta());
        if animation.timer.finished()
            { animation.index += 1; }

        let anim_vector = &animation.current_animation;
        *image = anim_vector[animation.index].clone();
    }
}