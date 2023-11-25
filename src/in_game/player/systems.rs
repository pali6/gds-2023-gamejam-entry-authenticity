use super::components::*;
use crate::in_game::animation::components::Animation;
use crate::in_game::animation::resources::AnimationResource;
use crate::in_game::inworld_object::InWorldObject;
use crate::utilities::*;
use bevy::prelude::*;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    world_params: Res<crate::world::WorldParams>,
    animation_resource: Res<AnimationResource>
) {
    let (spawn_x, spawn_y) = get_random_coords_padding(world_params.width, world_params.height, 50.0, 50.0);

    commands.spawn((
        // Sprite bundle contains most of the things
        // you need for a simple moving object on a screen
        SpriteBundle {
            transform: Transform::from_xyz(spawn_x, spawn_y, 0.0),
            texture: asset_server.load("sprites/Pizza.png"),
            ..default()
        },
        Animation::new(
            animation_resource.frame_period,
            animation_resource.rotating_pizza.clone(),
        ),
        InWorldObject,
        // Basically tag the sprite as the player
        Player,
    ));
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    // Just Skip if player not present
    // so the game goes on with player dead or absent
    let mut player_transform = match player_query.get_single_mut() {
        Ok(transform) => transform,
        Err(_e) => return,
    };

    // See `utilities` for `get_direction`
    let direction = get_direction(keyboard_input);
    player_transform.translation += direction * Player::MOVEMENT_SPEED * time.delta_seconds();
}
