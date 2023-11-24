
use bevy::prelude::*;
use bevy::core_pipeline::clear_color::ClearColorConfig;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Resource)]
#[allow(dead_code)]
pub enum CameraScalingMode {
    None,
    ScaleBoth,
    FitBoth,
    FitWidth,
    FitHeight,
}

pub struct CameraPlugin {
    pub scaling_mode: CameraScalingMode,
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(self.scaling_mode)
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, window_resize_camera_update);
    }
}

// Just a basic 2D camera
fn spawn_camera(mut commands: Commands) {
    let background_color = Color::rgb_u8(18, 18, 18);

    let mut camera = Camera2dBundle { ..default() };
    camera.camera_2d.clear_color = ClearColorConfig::Custom(background_color);

    commands.spawn(camera);
}

fn window_resize_camera_update(
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    world_size: Res<crate::world::WorldParams>,
    camera_scaling_mode: Res<CameraScalingMode>,
    mut resize_reader: EventReader<bevy::window::WindowResized>,
) {
    if let Some(event) = resize_reader.read().last() {
        let mut camera_transform = match camera_query.get_single_mut() {
            Ok(transform) => transform,
            Err(_e) => return,
        };

        let x: f32 = world_size.width / 2.0;
        let y: f32 = world_size.height / 2.0;
        let z: f32 = 0.0;

        camera_transform.translation = Vec3::new(x, y, z);

        camera_transform.scale = match *camera_scaling_mode {
            CameraScalingMode::None => Vec3::ONE,
            CameraScalingMode::ScaleBoth => Vec3::new(
                world_size.width / event.width,
                world_size.height / event.height,
                1.0,
            ),
            CameraScalingMode::FitBoth => {
                let scale = f32::max(
                    world_size.width / event.width,
                    world_size.height / event.height,
                );
                Vec3::new(scale, scale, 1.0)
            },
            CameraScalingMode::FitWidth => {
                let scale = world_size.width / event.width;
                Vec3::new(scale, scale, 1.0)
            },
            CameraScalingMode::FitHeight => {
                let scale = world_size.height / event.height;
                Vec3::new(scale, scale, 1.0)
            }
        }
    }
}