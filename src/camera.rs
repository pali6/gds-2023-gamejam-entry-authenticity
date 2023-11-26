use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

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
        app.insert_resource(self.scaling_mode)
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, window_resize_camera_update);
    }
}

// Just a basic 2D camera
fn spawn_camera(
    mut commands: Commands,
    world_params: Res<crate::world::WorldParams>,
    camera_scaling_mode: Res<CameraScalingMode>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let background_color = Color::rgb_u8(50, 140, 50);

    let transform = update_camera(
        &world_params,
        &camera_scaling_mode,
        window.single().width() as f32,
        window.single().height() as f32,
    );

    let mut camera = Camera2dBundle {
        transform,
        ..default()
    };
    camera.camera_2d.clear_color = ClearColorConfig::Custom(background_color);

    commands.spawn(camera);
}

fn window_resize_camera_update(
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    world_params: Res<crate::world::WorldParams>,
    camera_scaling_mode: Res<CameraScalingMode>,
    mut resize_reader: EventReader<bevy::window::WindowResized>,
) {
    if let Some(event) = resize_reader.read().last() {
        let mut camera_transform = match camera_query.get_single_mut() {
            Ok(transform) => transform,
            Err(_e) => return,
        };

        let width = event.width as f32;
        let height = event.height as f32;

        *camera_transform = update_camera(&world_params, &camera_scaling_mode, width, height);
    }
}

fn update_camera(
    world_params: &crate::world::WorldParams,
    camera_scaling_mode: &CameraScalingMode,
    width: f32,
    height: f32,
) -> Transform {
    let mut camera_transform = Transform::default();

    let x: f32 = world_params.width / 2.0;
    let y: f32 = world_params.height / 2.0;
    let z: f32 = 0.0;

    camera_transform.translation = Vec3::new(x, y, z);

    camera_transform.scale = match *camera_scaling_mode {
        CameraScalingMode::None => Vec3::ONE,
        CameraScalingMode::ScaleBoth => Vec3::new(
            world_params.width / width,
            world_params.height / height,
            1.0,
        ),
        CameraScalingMode::FitBoth => {
            let scale = f32::max(
                world_params.width / width,
                world_params.height / height,
            );
            Vec3::new(scale, scale, 1.0)
        }
        CameraScalingMode::FitWidth => {
            let scale = world_params.width / width;
            Vec3::new(scale, scale, 1.0)
        }
        CameraScalingMode::FitHeight => {
            let scale = world_params.height / height;
            Vec3::new(scale, scale, 1.0)
        }
    };

    camera_transform
}