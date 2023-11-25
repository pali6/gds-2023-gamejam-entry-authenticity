use bevy::prelude::*;
use crate::world::WorldParams;
use super::components::*;

pub fn spawn_static_objects(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    world_params: Res<WorldParams>,
) {
    let wheat_pos = world_params.wheat_location;
    let shed_pos = world_params.shed_location;

    // spawn the wheat
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(wheat_pos.x, wheat_pos.y, wheat_pos.z),
            texture: asset_server.load("sprites/wheat.png"),
            ..default()
        },
        StaticObject { }
    ));

    // spawn the shed
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(shed_pos.x, shed_pos.y, shed_pos.z),
            texture: asset_server.load("sprites/shed.png"),
            ..default()
        },
        StaticObject { }
    ));
}