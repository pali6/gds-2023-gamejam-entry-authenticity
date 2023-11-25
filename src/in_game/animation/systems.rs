use bevy::prelude::*;

use super::{components::*, resources::AnimationResource};

pub fn load_animation_resources(
    mut animation_resource: ResMut<AnimationResource>,
    asset_server: Res<AssetServer>
) {
    animation_resource.frame_period = 0.2;

    let vec = &mut animation_resource.rotating_pizza;
    vec.push(asset_server.load("sprites/rotating_pizza1.png"));
    vec.push(asset_server.load("sprites/rotating_pizza2.png"));
    vec.push(asset_server.load("sprites/rotating_pizza3.png"));
    vec.push(asset_server.load("sprites/rotating_pizza4.png"));

    // TODO
    //animation_resource.hen_idle.push(asset_server.load("sprites/idle1.jpg"));
}

pub fn update_animations(
    mut animation_query: Query<(&mut Animation, &mut Handle<Image>)>,
    time: Res<Time>
) {
    for (mut animation, mut image) in animation_query.iter_mut() {
        animation.timer.tick(time.delta());

        let anim_vector = &animation.current_animation;
        let anim_length: usize = anim_vector.len();
        *image = anim_vector[animation.index].clone();

        if animation.timer.finished() {
            animation.index += 1;
            animation.index %= anim_length;
        }
    }
}