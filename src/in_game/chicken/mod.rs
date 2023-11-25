pub mod components;
mod info_menu;
mod quirk;
mod resources;
mod systems;
mod click;
mod debug;

use super::states::InGameState;
use crate::states::AppState;
use bevy::prelude::*;
use info_menu::*;
use click::*;
use resources::*;
use systems::*;

pub struct ChickenPlugin;

impl Plugin for ChickenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChickenParams>()
            // This will restart the game every time
            // we go back from the menu though :(
            .add_systems(OnEnter(AppState::InGame), spawn_chickens)
            .add_event::<click::ChickenClickEvent>()
            .add_plugins(InfoMenuPlugin)
            // .add_systems(Update, debug::debug_chicken_click)
            .add_systems(
                Update,
                (chicken_movement, chicken_click)
                    .run_if(in_state(AppState::InGame).and_then(in_state(InGameState::Running))),
            );
    }
}
