use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum InGameState {
    #[default]
    Running,
    Paused,
}

pub fn toggle_pause(
    mut in_game_next_state: ResMut<NextState<InGameState>>,
    in_game_state: Res<State<InGameState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if !keyboard_input.just_pressed(KeyCode::P) {
        return;
    }

    if *in_game_state.get() == InGameState::Running {
        in_game_next_state.set(InGameState::Paused);
    }

    if *in_game_state.get() == InGameState::Paused {
        in_game_next_state.set(InGameState::Running);
    }
}

pub fn on_game_start(mut next_in_game_state: ResMut<NextState<InGameState>>) {
    next_in_game_state.set(InGameState::Running);
}
