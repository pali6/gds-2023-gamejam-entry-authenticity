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
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(22.0, 20.0), 18, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    animation_resource.bubble_atlas = texture_atlas_handle;

    let texture_handle = asset_server.load("sprites/smoke-2-Sheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 9, 1, Some(Vec2 { x: 1.0, y: 0.0 }), Some(Vec2{x: 0.5, y: 0.5}));
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    animation_resource.smoke_atlas = texture_atlas_handle;
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

pub fn update_scale_tween(
    mut commands: Commands,
    mut query: Query<(Entity, &mut ScaleTween, &mut Transform)>,
    time: Res<Time>
) {
    for (entity, mut scale_tween, mut transform) in query.iter_mut() {
        scale_tween.time += time.delta_seconds();
        let t = scale_tween.time / scale_tween.duration;
        
        let t_eased = scale_tween.easing.ease(t);

        let scale = scale_tween.from + (scale_tween.to - scale_tween.from) * t_eased;
        transform.scale = scale;

        if t >= 1.0 {
            commands.entity(entity).remove::<ScaleTween>();
        }
    }
}

pub fn update_fade_away_tween(
    mut commands: Commands,
    mut query: Query<(Entity, &mut FadeAwayTween, &mut Sprite)>,
    time: Res<Time>
) {
    for (entity, mut f_tween, mut image) in query.iter_mut() {

        f_tween.time += time.delta_seconds();
        let t = ((f_tween.time - f_tween.delay) / f_tween.duration).max(0.0);
        let t_eased = f_tween.easing.ease(t);

        image.color.set_a(1.0 - t_eased);

        if t >= 1.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn update_fade_away_tween_atlas(
    mut commands: Commands,
    mut query: Query<(Entity, &mut FadeAwayTween, &mut TextureAtlasSprite)>,
    time: Res<Time>
) {
    for (entity, mut f_tween, mut image) in query.iter_mut() {

        f_tween.time += time.delta_seconds();
        let t = ((f_tween.time - f_tween.delay) / f_tween.duration).max(0.0);
        let t_eased = f_tween.easing.ease(t);

        image.color.set_a(1.0 - t_eased);

        if t >= 1.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}