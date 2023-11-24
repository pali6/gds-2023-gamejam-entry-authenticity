mod player;
mod states;

use bevy::prelude::*;
use crate::states::AppState;
use player::PlayerPlugin;
use states::*;


pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<InGameState>()
            .add_plugins(PlayerPlugin)
            .add_systems(Update, toggle_pause.run_if(in_state(AppState::InGame)));
    }
}