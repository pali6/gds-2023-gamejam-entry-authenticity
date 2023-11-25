use bevy::prelude::*;

use crate::utilities::{confine_movement, play_sfx};

#[derive(Component)]
pub struct InWorldObject;

pub fn confine_inworld_movement(
    mut commands: Commands,
    mut inworld_query: Query<(&mut Transform, &Handle<Image>), (With<Sprite>, With<InWorldObject>)>,
    world_params: Res<crate::world::WorldParams>,
    asset_server: Res<AssetServer>,
    assets: Res<Assets<Image>>,
) {
    for (mut transform, image_handle) in inworld_query.iter_mut() {
        let maybe_image = assets.get(image_handle);
        let image_dimensions = maybe_image
            .and_then(|image| Some(image.size()))
            .unwrap_or(UVec2::ZERO);
        let scaled_image_dimension = Vec2::new(
            image_dimensions.x as f32 * transform.scale.x,
            image_dimensions.y as f32 * transform.scale.y,
        );
        let pos = &mut transform.translation;

        let (new_x, new_y) = confine_movement(
            pos.x,
            pos.y,
            world_params.width,
            world_params.height,
            scaled_image_dimension.x,
            scaled_image_dimension.y,
        );

        if new_x != pos.x || new_y != pos.y {
            play_sfx("sounds/bop.ogg", &mut commands, &asset_server);
        }

        pos.x = new_x;
        pos.y = new_y;
    }
}

pub fn despawn_inworld_objects(
    mut commands: Commands,
    object_query: Query<Entity, With<InWorldObject>>,
) {
    for object in object_query.iter() {
        commands.entity(object).despawn();
    }
}
