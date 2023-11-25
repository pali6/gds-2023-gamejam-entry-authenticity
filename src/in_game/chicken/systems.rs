use std::borrow::BorrowMut;

use super::components::*;
use super::resources::ChickenParams;
use crate::in_game::inworld_object::InWorldObject;
use crate::utilities::*;
use crate::world::WorldParams;
use bevy::prelude::*;
use crate::one_shot::*;

pub fn spawn_chickens(mut commands: Commands) {
    for _ in 0..10 { // TODO: un-hardcode this
        commands.run_once(spawn_chicken);
    }
}

pub fn spawn_chicken(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut chicken_params: ResMut<ChickenParams>,
    world_params: Res<WorldParams>,
) {
    let (spawn_x, spawn_y) = get_random_coords(world_params.width, world_params.height);

    commands.spawn((
        chicken_params.get_random_chicken_bundle(spawn_x, spawn_y, &asset_server),
        InWorldObject,
        Chicken::new_random(chicken_params.borrow_mut()),
    ));
}

pub fn chicken_movement(
    mut chicken_query: Query<(&mut Transform, &Chicken)>,
    time: Res<Time>,
) {
    for (mut chicken_transform, chicken) in chicken_query.iter_mut() {
        let direction = Dir::random().to_vector();
        chicken_transform.translation += direction * chicken.movement_speed * time.delta_seconds();
    }
}

pub fn despawn_chickens(mut commands: Commands, chicken_query: Query<Entity, With<Chicken>>) {
    for chicken_entity in chicken_query.iter() {
        commands.entity(chicken_entity).despawn();
    }
}
