pub mod components;
mod info_menu;
pub mod quirk;
pub mod resources;
mod systems;
mod click;
mod debug;
mod hover_glow;
pub mod kill_click;
pub mod chicken_hunt;

use self::components::ChickenParts;

use super::states::InGameState;
use crate::states::AppState;
use bevy::prelude::*;
use info_menu::*;
use click::*;
use resources::*;
use systems::*;
use chicken_hunt::*;

pub struct ChickenPlugin;

impl Plugin for ChickenPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ChickenParams>()
            .init_resource::<HoveredOverChicken>()
            .init_resource::<ChickenAtlas>()
            // This will restart the game every time
            // we go back from the menu though :(
            .add_systems(Startup, ChickenParts::add_chicken_parts_to_atlas)
            .add_systems(OnEnter(AppState::InGame), (spawn_chickens).after(ChickenParts::add_chicken_parts_to_atlas))
            .add_event::<click::ChickenClickEvent>()
            .add_systems(PreUpdate, chicken_click)
            // .add_systems(Update, debug::debug_chicken_click)
            .add_systems(
                Update,
                (kill_click::click_kill, chicken_hover)
                    .run_if(in_state(AppState::InGame).and_then(in_state(InGameState::Running))),
            )
            .add_plugins(InfoMenuPlugin)
            .add_plugins(hover_glow::HoverGlowPlugin)
            .add_plugins(ChickenHuntPlugin);
    }
}
