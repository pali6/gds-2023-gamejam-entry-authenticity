use bevy::prelude::*;
use crate::world::WorldParams;
use super::components::*;
use super::super::chicken::resources::*;

pub fn spawn_static_objects(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    world_params: Res<WorldParams>,
    chicken_atlas: ResMut<ChickenAtlas>,
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
            texture: asset_server.load("sprites/shed_shadow.png"),
            ..default()
        },
        StaticObject { }
    ));
    
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(shed_pos.x, shed_pos.y, shed_pos.z + 1.0),
            texture: asset_server.load("sprites/shed_side_walls.png"),
            ..default()
        },
        StaticObject { }
    ));
    
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(shed_pos.x, shed_pos.y, shed_pos.z + 2.0),
            texture: asset_server.load("sprites/shed_roof.png"),
            ..default()
        },
        StaticObject { }
    ));
}