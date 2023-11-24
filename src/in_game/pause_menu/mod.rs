mod systems;

use crate::in_game::states::InGameState;
use bevy::prelude::*;
use systems::*;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(InGameState::Paused), build_pause_menu)
            .add_systems(OnExit(InGameState::Paused), destroy_pause_menu)
            .add_systems(Update, (ui_pause_menu,).run_if(run_if_in_pause_menu));
    }
}
