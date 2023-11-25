use bevy::prelude::*;

use super::{components::*, resources::AnimationResource};

pub fn load_animation_resources(
    mut animation_resource: ResMut<AnimationResource>,
    asset_server: Res<AssetServer>,
) {
    animation_resource.frame_period = 1.0;

    let vec = &mut animation_resource.rotating_pizza;
    vec.push(asset_server.load("sprites/rotating_pizza1.png"));
    vec.push(asset_server.load("sprites/rotating_pizza2.png"));
    vec.push(asset_server.load("sprites/rotating_pizza3.png"));
    vec.push(asset_server.load("sprites/rotating_pizza4.png"));

    // TODO
    //animation_resource.hen_idle.push(asset_server.load("sprites/idle1.jpg"));
}

pub fn update_animations(
    mut animation_query: Query<(&mut Animation, &mut TextureAtlasSprite)>,
    time: Res<Time>,
) {
    for (mut animation, mut sprite) in animation_query.iter_mut() {
        animation.timer.tick(time.delta());

        if animation.timer.just_finished() || animation.is_changed {

            let index_buffer = animation.index_buffer;
            let anim_length: usize = index_buffer.len();
            animation.frame %= anim_length;

            let index = index_buffer[animation.frame];
            *sprite = TextureAtlasSprite::new(index);

            animation.frame += 1;
            animation.is_changed = false;
        }
    }
}
