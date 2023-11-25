use bevy::prelude::*;

use super::{components::*, resources::AnimationResource};

pub fn load_animation_resources(
    mut animation_resource: ResMut<AnimationResource>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    animation_resource.frame_period = 1.0;

    let vec = &mut animation_resource.rotating_pizza;
    vec.push(asset_server.load("sprites/rotating_pizza1.png"));
    vec.push(asset_server.load("sprites/rotating_pizza2.png"));
    vec.push(asset_server.load("sprites/rotating_pizza3.png"));
    vec.push(asset_server.load("sprites/rotating_pizza4.png"));

    // TODO
    //animation_resource.hen_idle.push(asset_server.load("sprites/idle1.jpg"));
    let texture_handle = asset_server.load("sprites/speech_bubble_sheet2.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(11.0, 10.0), 18, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    animation_resource.bubble_atlas = texture_atlas_handle;
}

pub fn update_animations(
    mut animation_query: Query<(&mut Animation, &mut TextureAtlasSprite)>,
    time: Res<Time>,
) {
    for (mut animation, mut sprite) in animation_query.iter_mut() {
        animation.time += time.delta_seconds();

        if animation.time > animation.period || animation.is_changed {
            animation.time %= animation.period;

            let index_buffer = animation.index_buffer;
            let anim_length: usize = index_buffer.len();

            if animation.repeating || animation.frame < anim_length - 1 {
                animation.frame += 1;
            }

            if animation.repeating { 
                animation.frame %= anim_length;
            }

            let index = index_buffer[animation.frame];
            *sprite = TextureAtlasSprite::new(index);

            animation.is_changed = false;
        }
    }
}
