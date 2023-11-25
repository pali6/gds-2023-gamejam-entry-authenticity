use std::borrow::BorrowMut;

use super::components::*;
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

pub fn spawn_chickens(mut commands: Commands) {
    for _ in 0..10 {
        // TODO: un-hardcode this
        commands.run_once(spawn_chicken);
    }
}

pub fn spawn_chicken(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut chicken_params: ResMut<ChickenParams>,
    world_params: Res<WorldParams>,
    anim_resource: Res<AnimationResource>
) {
    let (spawn_x, spawn_y) = get_random_coords_padding(world_params.width, world_params.height, 50.0, 50.0);

    let chicken = Chicken::new_random(chicken_params.borrow_mut());
    let chicken_name = chicken.name.clone();


    let chicken_entity = commands
        .spawn((
            chicken_params.get_random_chicken_bundle(spawn_x, spawn_y, &asset_server),
            InWorldObject,
            chicken,
            Behavior::new(BehaviorType::RandomMovement),
            Animation::new(anim_resource.frame_period, anim_resource.rotating_pizza.clone())
        ))
        .id();

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
                transform: Transform::from_xyz(0.0, -25.0, 20.0),
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