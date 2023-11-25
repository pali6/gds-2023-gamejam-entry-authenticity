pub mod components;
mod systems;

use bevy::prelude::*;
use systems::*;

use crate::states::AppState;

use super::states::InGameState;

pub struct BehaviorPlugin;

impl Plugin for BehaviorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(Update, (
                update_chicken_behaviours,
                update_speech_bubbles
            ).run_if(in_state(AppState::InGame).and_then(in_state(InGameState::Running))));
    }
}