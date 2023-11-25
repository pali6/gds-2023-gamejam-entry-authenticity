use bevy::prelude::*;
use crate::world::WorldParams;
use super::components::*;
use super::resources::*;

pub fn load_static_object_assets(
    asset_server: Res<AssetServer>,
    mut static_object_res: ResMut<StaticObjectsResource>,
) {
    static_object_res.front_fence_handle = asset_server.load("sprites/front_fence.png");
    static_object_res.back_fence_handle = asset_server.load("sprites/back_fence.png");
    static_object_res.left_fence_handle = asset_server.load("sprites/left_fence.png");
    static_object_res.right_fence_handle = asset_server.load("sprites/right_fence.png");
}

pub fn spawn_static_objects(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    world_params: Res<WorldParams>,
    assets: Res<Assets<Image>>,
    static_object_res: Res<StaticObjectsResource>,
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

    let front_fence_handle = static_object_res.front_fence_handle.clone();
    let front_fence_image = assets.get(front_fence_handle.clone()).unwrap();
    let front_fence_dimensions = front_fence_image.size();
    for x in (0..world_params.width as usize).step_by(front_fence_dimensions.x as usize) {
        let pos_x = (x as f32 + front_fence_dimensions.x as f32 / 2.0).min(world_params.width - front_fence_dimensions.x as f32 / 2.0);
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(pos_x, front_fence_dimensions.y as f32 / 2.0, 20.0),
                texture: front_fence_handle.clone(),
                ..default()
            },
            StaticObject { }
        ));
    }

    let back_fence_handle = static_object_res.back_fence_handle.clone();
    let back_fence_image = assets.get(back_fence_handle.clone()).unwrap();
    let back_fence_dimensions = back_fence_image.size();
    for x in (0..world_params.width as usize).step_by(back_fence_dimensions.x as usize) {
        let pos_x = (x as f32 + back_fence_dimensions.x as f32 / 2.0).min(world_params.width - back_fence_dimensions.x as f32 / 2.0);
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(pos_x, world_params.height - back_fence_dimensions.y as f32 / 2.0, 0.0),
                texture: back_fence_handle.clone(),
                ..default()
            },
            StaticObject { }
        ));
    }

    let left_fence_handle = static_object_res.left_fence_handle.clone();
    let left_fence_image = assets.get(left_fence_handle.clone()).unwrap();
    let left_fence_dimensions = left_fence_image.size();
    for y in (0..world_params.height as usize).step_by(left_fence_dimensions.y as usize) {
        let pos_y = (y as f32 + left_fence_dimensions.y as f32 / 2.0).min(world_params.height - left_fence_dimensions.y as f32 / 2.0);
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(left_fence_dimensions.x as f32 / 2.0, pos_y, 0.0),
                texture: left_fence_handle.clone(),
                ..default()
            },
            StaticObject { }
        ));
    }

    let right_fence_handle = static_object_res.right_fence_handle.clone();
    let right_fence_image = assets.get(right_fence_handle.clone()).unwrap();
    let right_fence_dimensions = right_fence_image.size();
    for y in (0..world_params.height as usize).step_by(right_fence_dimensions.y as usize) {
        let pos_y = (y as f32 + right_fence_dimensions.y as f32 / 2.0).min(world_params.height - right_fence_dimensions.y as f32 / 2.0);
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(world_params.width - right_fence_dimensions.x as f32 / 2.0, pos_y, 0.0),
                texture: right_fence_handle.clone(),
                ..default()
            },
            StaticObject { }
        ));
    }
}

pub fn despawn_static_objects(
    mut commands: Commands,
    object_query: Query<Entity, With<StaticObject>>,
) {
    for entity in object_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}