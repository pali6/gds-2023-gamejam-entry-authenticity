use std::borrow::BorrowMut;

use super::components::*;
use super::resources::ChickenAtlas;
use super::resources::ChickenParams;
use crate::in_game::animation::components::Animation;
use crate::in_game::animation::resources::AnimationResource;
use crate::in_game::behavior::components::Behavior;
use crate::in_game::behavior::components::BehaviorType;
use crate::in_game::inworld_object::InWorldObject;
use crate::one_shot::*;
use crate::utilities::*;
use crate::world::WorldParams;
use bevy::prelude::*;
use rand::Rng;
use rand::seq::SliceRandom;

pub fn spawn_chickens(
    mut commands: Commands,
    mut world_params: ResMut<WorldParams>,
    mut chicken_params: ResMut<ChickenParams>,
) {
    let total_count = world_params.chicken_count + world_params.fox_count;
    chicken_params.n_foxes_to_spawn = world_params.fox_count;
    for _ in 0..total_count {
        commands.run_once(spawn_chicken);
    }
    world_params.chicken_alive = world_params.chicken_count;
    world_params.foxes_alive = world_params.fox_count;
}

pub fn spawn_chicken(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut chicken_params: ResMut<ChickenParams>,
    chicken_atlas: ResMut<ChickenAtlas>,
    world_params: Res<WorldParams>,
    anim_resource: Res<AnimationResource>
) {
    let (spawn_x, spawn_y) = get_random_coords_padding(world_params.width, world_params.height, 50.0, 50.0);

    let mut chicken = Chicken::new_random(chicken_params.borrow_mut(), world_params.quirks_per_chicken);
    chicken.is_fox = if chicken_params.n_foxes_to_spawn > 0 {
        chicken_params.n_foxes_to_spawn -= 1;
        true
    } else {
        false
    };
    let chicken_name = chicken.name.clone();

    let chicken_entity = commands
        .spawn((
            chicken_params.get_random_chicken_bundle(spawn_x, spawn_y, &asset_server),
            //Transform::from_xyz(spawn_x, spawn_y, 0.0),
            InWorldObject,
            chicken,
            Behavior::new(BehaviorType::RandomMovement),
        ))
        .id();

    if chicken_atlas.sprite_sheets.len() > 0 {
        let mut rng = rand::thread_rng();

        let mut chaos_chicken = false;
        // chaos chicken
        if rand::thread_rng().gen_range(0.0..1.0) >= 0.95 {
            chaos_chicken = true;
        }

        let mut chicken_atlas_handle = chicken_atlas.sprite_sheets.choose(&mut rng).unwrap();     
        let mut parts = ChickenParts::new_idle(chicken_atlas_handle.clone());
        
        let body = commands.spawn((
            parts.body,
            ChickenPart,
            Animation::new_chicken(anim_resource.frame_period,BodyPart::Body)))
        .id();

        if chaos_chicken {
            chicken_atlas_handle = chicken_atlas.sprite_sheets.choose(&mut rng).unwrap();     
            parts = ChickenParts::new_idle(chicken_atlas_handle.clone());
        }

        let tail = commands.spawn((
            parts.tail,
            ChickenPart,
            Animation::new_chicken(anim_resource.frame_period,BodyPart::Tail)))
        .id();

        if chaos_chicken {
            chicken_atlas_handle = chicken_atlas.sprite_sheets.choose(&mut rng).unwrap();     
            parts = ChickenParts::new_idle(chicken_atlas_handle.clone());
        }

        let wing = commands.spawn((
            parts.wing,
            ChickenPart,
            Animation::new_chicken(anim_resource.frame_period,BodyPart::Wing)))
        .id();

        if chaos_chicken {
            chicken_atlas_handle = chicken_atlas.sprite_sheets.choose(&mut rng).unwrap();     
            parts = ChickenParts::new_idle(chicken_atlas_handle.clone());
        }

        let head = commands.spawn((
            parts.head,
            ChickenPart,
            Animation::new_chicken(anim_resource.frame_period,BodyPart::Head)))
        .id();

        commands.entity(chicken_entity).push_children(&[body, tail, wing, head]);
    } else {
        panic!("WHAAAAAAAAAT");
    }

    let nametag = commands
        .spawn(
            Text2dBundle {
                text: Text::from_section(
                    chicken_name,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"), // TODO: Less serious font. Random fonts to give the chickens personality?
                        font_size: 16.0,
                        color: Color::rgba(1.0, 1.0, 1.0, 0.7),
                    },
                )
                .with_alignment(TextAlignment::Center),
                transform: Transform::from_xyz(0.0, -25.0, 420.0),
                ..Default::default()
            },
            // TODO: update name dynamically mayhaps?
            // TODO: we could check if out of screen and move the name to fit but probably not worth it
        )
        .id();

    commands.entity(chicken_entity).push_children(&[nametag]);
}

pub fn chicken_movement(mut chicken_query: Query<(&mut Transform, &Chicken)>, time: Res<Time>) {
    for (mut chicken_transform, chicken) in chicken_query.iter_mut() {
        let direction = Dir::random().to_vector();
        chicken_transform.translation += direction * chicken.movement_speed * time.delta_seconds();
    }
}