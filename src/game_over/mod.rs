mod systems;

use crate::states::AppState;
use bevy::prelude::*;
use systems::*;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), build_game_over)
            .add_systems(OnExit(AppState::GameOver), destroy_game_over)
            .add_systems(Update, (ui_game_over,).run_if(in_state(AppState::GameOver)));
    }
}