use bevy::prelude::*;

use crate::{states::AppState, utilities::get_random_coords_padding};

use super::inworld_object::InWorldObject;

#[derive(Component)]
pub struct Nest;

pub struct NestPlugin;

impl Plugin for NestPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::InGame), spawn_nests);
    }
}

fn spawn_nests(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut world_params: ResMut<crate::world::WorldParams>,
) {
    let nest_texture_handle = asset_server.load("sprites/nest.png");
    let mut used_coords = Vec::new();
    world_params.nest_locations.clear();
    for _ in 0..world_params.nest_count {
        let (spawn_x, spawn_y) = loop {
            let (x, y) = get_random_coords_padding(world_params.width, world_params.height, 50.0, 50.0);
            let pos = Vec2::new(x, y);
            if x > world_params.width / 2.0 - 150.0 && x < world_params.width / 2.0 + 150.0 {
                continue;
            }
            if !used_coords.iter().any(|&coord: &Vec2| coord.distance(pos) < 100.0) {
                used_coords.push(pos);
                break (x, y);
            }
        };

        world_params.nest_locations.push((spawn_x, spawn_y));
        commands.spawn((
            SpriteBundle {
                texture: nest_texture_handle.clone(),
                transform: Transform::from_xyz(spawn_x, spawn_y, 0.0),
                ..Default::default()
            },
            Nest,
            InWorldObject,
        ));
    }
}