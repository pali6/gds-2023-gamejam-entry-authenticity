mod in_game;
mod main_menu;
mod states;
mod utilities;

use bevy::app::AppExit;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use in_game::InGamePlugin;
use main_menu::MainMenuPlugin;
use states::AppState;

fn main() {
    App::new()
        // AppState will be accessible as a resource
        // This allows to switch into MainMenu etc.
        .add_state::<AppState>()

        // Modulate your game into plugins
        .add_plugins((
            // Provided by bevy. Spawns window and stuff...
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Game Title".to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            // Made by me
            InGamePlugin,
            MainMenuPlugin
        ))

        // Systems -> App starts
        .add_systems(Startup, (spawn_camera,))

        // Systems -> Every frame
        .add_systems(Update, (
            exit_game,
            toggle_app_state
        ))

        // Don't forget to run the app :]
        .run();
}

// Just a basic 2D camera
fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();
    let background_color = Color::rgb_u8(18, 18, 18);
    let x: f32 = window.width() / 2.0;
    let y: f32 = window.height() / 2.0;
    let z: f32 = 0.0;

    let mut camera = Camera2dBundle { ..default() };
    camera.transform = Transform::from_xyz(x, y, z);
    camera.camera_2d.clear_color = ClearColorConfig::Custom(background_color);

    commands.spawn(camera);
}

fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>
) {
    if keyboard_input.just_pressed(KeyCode::F4) {
        app_exit_event_writer.send(AppExit);
    }
}

fn toggle_app_state(
    mut app_next_state: ResMut<NextState<AppState>>,
    app_state: Res<State<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if !keyboard_input.just_pressed(KeyCode::Escape)
        { return; }

    if *app_state.get() == AppState::InGame
        { app_next_state.set(AppState::MainMenu); }
    
    if *app_state.get() == AppState::MainMenu
        { app_next_state.set(AppState::InGame); }
}