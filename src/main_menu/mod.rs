mod systems;

use crate::states::AppState;
use bevy::prelude::*;
use systems::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::MainMenu), build_main_menu)
            .add_systems(OnExit(AppState::MainMenu), destroy_main_menu)
            .add_systems(
                Update,
                (ui_main_menu,).run_if(run_if_in_main_menu),
            );
    }
}