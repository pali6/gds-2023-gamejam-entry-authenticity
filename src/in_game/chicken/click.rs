use bevy::{prelude::*, window::PrimaryWindow, window::Window};

use super::components::Chicken;

#[derive(Event)]
pub struct ChickenClickEvent {
    pub chicken: Entity,
    pub mouse_button: Input<MouseButton>,
}

#[derive(Resource, Default, Debug)]
pub struct HoveredOverChicken {
    pub chicken: Option<Entity>,
}

pub fn chicken_hover(
    chicken_query: Query<(Entity, &Transform, &Handle<Image>), (With<Sprite>, With<Chicken>)>,
    windows: Query<&Window, With<PrimaryWindow>>,
    assets: Res<Assets<Image>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut hovered_over_chicken: ResMut<HoveredOverChicken>,
    menus: Res<crate::in_game::chicken::info_menu::ChickenMenus>,
) {
    let Some(viewport_mouse_position) = windows.get_single().ok().and_then(Window::cursor_position)
    else {
        hovered_over_chicken.chicken = None;
        return;
    };
    let (camera, camera_transform) = camera_query.single();
    let Some(world_mouse_position) =
        camera.viewport_to_world_2d(camera_transform, viewport_mouse_position)
    else {
        hovered_over_chicken.chicken = None;
        return;
    };

    if menus.mouse_over_menu {
        hovered_over_chicken.chicken = None;
        return;
    }

    let mut closest_chicken: Option<Entity> = None;
    let mut closest_distance: f32 = f32::INFINITY;

    for (entity, transform, image_handle) in chicken_query.iter() {
        let maybe_image = assets.get(image_handle);
        let image_dimensions = maybe_image
            .and_then(|image| Some(image.size()))
            .unwrap_or(UVec2::ZERO);
        let scaled_image_dimension = Vec2::new(
            image_dimensions.x as f32 * transform.scale.x,
            image_dimensions.y as f32 * transform.scale.y,
        );

        let sprite_left = transform.translation.x - scaled_image_dimension.x / 2.0;
        let sprite_right = transform.translation.x + scaled_image_dimension.x / 2.0;
        let sprite_top = transform.translation.y + scaled_image_dimension.y / 2.0;
        let sprite_bottom = transform.translation.y - scaled_image_dimension.y / 2.0;

        if world_mouse_position.x >= sprite_left
            && world_mouse_position.x <= sprite_right
            && world_mouse_position.y <= sprite_top
            && world_mouse_position.y >= sprite_bottom
        {
            let distance = (world_mouse_position - transform.translation.xy()).length();

            if distance < closest_distance {
                closest_distance = distance;
                closest_chicken = Some(entity);
            }
        }
    }

    hovered_over_chicken.chicken = closest_chicken;
}

pub fn chicken_click(
    buttons: Res<Input<MouseButton>>,
    hovered_over_chicken: Res<HoveredOverChicken>,
    mut chicken_click_events: EventWriter<ChickenClickEvent>,
) {
    if buttons.get_just_pressed().count() == 0 && buttons.get_just_released().count() == 0 {
        return;
    }

    if let Some(entity) = hovered_over_chicken.chicken {
        chicken_click_events.send(ChickenClickEvent {
            chicken: entity,
            mouse_button: buttons.clone(),
        });
    }
}
