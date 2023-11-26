use bevy::{app::AppExit, prelude::*};
use crate::in_game::states::InGameState;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver
}

pub fn toggle_app_state(
    // mut app_next_state: ResMut<NextState<AppState>>,
    ingame_state: Res<State<InGameState>>,
    mut ingame_next_state: ResMut<NextState<InGameState>>,
    app_state: Res<State<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if *app_state.get() == AppState::InGame {
            if *ingame_state.get() == InGameState::Running {
                ingame_next_state.set(InGameState::Paused);
            } else if *ingame_state.get() == InGameState::Paused {
                ingame_next_state.set(InGameState::Running);
            }
        } else if *app_state.get() == AppState::MainMenu {
            app_exit_event_writer.send(AppExit);
        }
    }
}
