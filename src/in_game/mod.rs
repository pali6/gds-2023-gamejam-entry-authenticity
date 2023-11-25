mod animation;
mod chicken;
pub mod inworld_object;
pub mod pause_menu;
mod player;
mod states;

use crate::states::AppState;
use animation::AnimationPlugin;
use bevy::prelude::*;
use chicken::ChickenPlugin;
use inworld_object::*;
use player::PlayerPlugin;
use states::*;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<InGameState>()
            .add_plugins((PlayerPlugin, AnimationPlugin, ChickenPlugin))
            .add_systems(Update, toggle_pause.run_if(in_state(AppState::InGame)))
            .add_systems(OnEnter(AppState::InGame), on_game_start)
            .add_systems(OnExit(AppState::InGame), despawn_inworld_objects)
            .add_systems(PostUpdate, confine_inworld_movement);
    }
}
