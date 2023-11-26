mod camera;
mod cursor;
mod in_game;
mod main_menu;
mod one_shot;
mod states;
mod utilities;
mod world;
mod help;
mod timed_sounds;
mod ambience;

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_embedded_assets::EmbeddedAssetPlugin;

use in_game::pause_menu::PauseMenuPlugin;
use in_game::InGamePlugin;
use main_menu::MainMenuPlugin;
use states::*;

fn main() {
    let width = 1920.0 / 2.0;
    let height = 1080.0 / 2.0;

    App::new()
        // AppState will be accessible as a resource
        // This allows to switch into MainMenu etc.
        .add_state::<AppState>()
        .insert_resource(world::WorldParams {
            width: width,
            height: height,
            wheat_location: Vec3::new(width / 2.0, height / 4.0, 0.0), // bottom middle
            shed_location: Vec3::new(width / 2.0, height * 3.0 / 4.0, 15.0), // upper middle
            quirks_per_chicken: 3,
            chicken_count: 20,
            fox_count: 2,
            nest_count: 5,
            nest_locations: Vec::new()
        })
        // Modulate your game into plugins
        .add_plugins((
            EmbeddedAssetPlugin {
                mode: bevy_embedded_assets::PluginMode::ReplaceDefault,
            },
            // Provided by bevy. Spawns window and stuff...
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "aut-HEN-tic".to_string(),
                    resolution: bevy::window::WindowResolution::new(width, height),
                    present_mode: bevy::window::PresentMode::AutoNoVsync,
                    fit_canvas_to_parent: true,
                    ..Default::default()
                }),
                ..Default::default()
            }).set(ImagePlugin::default_nearest()),
            EguiPlugin,
            // Made by me
            InGamePlugin,
            MainMenuPlugin,
            PauseMenuPlugin,
            timed_sounds::TimedSoundsPlugin,
            ambience::AmbiencePlugin,
            camera::CameraPlugin {
                scaling_mode: camera::CameraScalingMode::FitBoth,
            },
            cursor::CursorPlugin {},
        ))
        // Systems -> Every frame
        .add_systems(Update, (exit_game, toggle_app_state))
        // Don't forget to run the app :]
        .run();
}

fn exit_game(keyboard_input: Res<Input<KeyCode>>, mut app_exit_event_writer: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::F4) {
        app_exit_event_writer.send(AppExit);
    }
}

#[allow(dead_code)]
fn spritemap_fix(
    mut ev_asset: EventReader<AssetEvent<Image>>,
    mut assets: ResMut<Assets<Image>>,
) {
    for ev in ev_asset.read() {
        match ev {
            AssetEvent::Added { id } => {
                if let Some(texture) = assets.get_mut(*id) {
                    texture.sampler = bevy::render::texture::ImageSampler::nearest()
                }
            },
            _ => {}
        }
    }
}