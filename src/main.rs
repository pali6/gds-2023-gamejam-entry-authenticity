mod in_game;
mod main_menu;
mod states;
mod utilities;
mod world;
mod camera;
mod one_shot;

use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_embedded_assets::EmbeddedAssetPlugin;

use in_game::pause_menu::PauseMenuPlugin;
use in_game::InGamePlugin;
use main_menu::MainMenuPlugin;
use states::*;

fn main() {
    App::new()
        // AppState will be accessible as a resource
        // This allows to switch into MainMenu etc.
        .add_state::<AppState>()
        .insert_resource(world::WorldParams {
            width: 1920.0 / 2.0,
            height: 1080.0 / 2.0,
        })
        // Modulate your game into plugins
        .add_plugins((
            EmbeddedAssetPlugin { mode: bevy_embedded_assets::PluginMode::ReplaceDefault },
            // Provided by bevy. Spawns window and stuff...
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "aut-HEN-tic".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            EguiPlugin,
            // Made by me
            InGamePlugin,
            MainMenuPlugin,
            PauseMenuPlugin,
            camera::CameraPlugin { scaling_mode: camera::CameraScalingMode::FitBoth },
        ))
        // Systems -> Every frame
        .add_systems(Update, (exit_game, toggle_app_state))
        .add_systems(PostUpdate, in_game::inworld_object::confine_inworld_movement)
        // Don't forget to run the app :]
        .run();
}

fn exit_game(keyboard_input: Res<Input<KeyCode>>, mut app_exit_event_writer: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::F4) {
        app_exit_event_writer.send(AppExit);
    }
}
