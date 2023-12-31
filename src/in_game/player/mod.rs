pub mod components;
mod resources;
mod systems;

use super::states::InGameState;
use crate::states::AppState;
use bevy::prelude::*;
use resources::*;
use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ExamplePlayerResource>()
            // This will restart the game every time
            // we go back from the menu though :(
            .add_systems(OnEnter(AppState::InGame), spawn_player)
            .add_systems(
                Update,
                (player_movement,)
                    // I have not found an easier way to do this...
                    // Every plugin in the InGame mod will have to
                    // handle it's systems, in which state should they run
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(InGameState::Running)),
            );
    }
}
