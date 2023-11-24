use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default] MainMenu,
    InGame,
    // GameOver next?
}

pub fn toggle_app_state(
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