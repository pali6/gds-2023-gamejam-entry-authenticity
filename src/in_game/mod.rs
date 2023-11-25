pub mod pause_menu;
mod player;
mod states;
mod animation;
mod chicken;
pub mod inworld_object;

use crate::states::AppState;
use bevy::prelude::*;
use player::PlayerPlugin;
use states::*;
use animation::AnimationPlugin;
use chicken::ChickenPlugin;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<InGameState>()
            .add_plugins((PlayerPlugin, AnimationPlugin, ChickenPlugin))
            .add_systems(Update, toggle_pause.run_if(in_state(AppState::InGame)))
            .add_systems(OnEnter(AppState::InGame), on_game_start);
    }
}
