mod systems;

use crate::states::AppState;
use bevy::prelude::*;
use systems::*;

// TODO: I have not yet gotten to creating the UI

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::MainMenu), build_menu);
    }
}