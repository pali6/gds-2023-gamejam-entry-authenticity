use super::components::*;
use crate::utilities::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();

    let (spawn_x, spawn_y) = get_random_coords(window.width(), window.height());

    commands.spawn((
        // Sprite bundle contains most of the things
        // you need for a simple moving object on a screen
        SpriteBundle {
            transform: Transform::from_xyz(spawn_x, spawn_y, 0.0),
            texture: asset_server.load("sprites/Pizza.png"),
            ..default()
        },
        // Basically tag the sprite as the player
        Player {},
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

// Basically collision resolution
pub fn confine_player_movement(
    mut commands: Commands,
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let mut player_transform = match player_query.get_single_mut() {
        Ok(transform) => transform,
        Err(_e) => return,
    };

    // Just rename it, so it's shorter...
    let pos = &mut player_transform.translation;

    // see `utilities` for `confine_movement`
    let (new_x, new_y) =
        confine_movement(pos.x, pos.y, window.width(), window.height(), Player::SIZE);

    // Just an example of sound usage,
    // when player hits the edge.
    // Maybe not the best example :D
    if new_x != pos.x || new_y != pos.y {
        play_sfx("sounds/bop.ogg", &mut commands, &asset_server);
    }

    pos.x = new_x;
    pos.y = new_y;
}

pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    let player_entity = match player_query.get_single() {
        Ok(entity) => entity,
        Err(_e) => return,
    };

    commands.entity(player_entity).despawn();
}
