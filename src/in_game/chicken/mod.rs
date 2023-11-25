pub mod components;
mod systems;
mod quirk;
mod resources;

use super::states::InGameState;
use crate::states::AppState;
use bevy::prelude::*;
use resources::*;
use systems::*;

pub struct ChickenPlugin;

impl Plugin for ChickenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChickenParams>()
            // This will restart the game every time
            // we go back from the menu though :(
            .add_systems(OnEnter(AppState::InGame), spawn_chickens)
            .add_systems(OnExit(AppState::InGame), despawn_chickens)
            .add_systems(
                Update,
                (
                    chicken_movement,
                )
                    // I have not found an easier way to do this...
                    // Every plugin in the InGame mod will have to
                    // handle it's systems, in which state should they run
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(InGameState::Running)),
            );
    }
}
