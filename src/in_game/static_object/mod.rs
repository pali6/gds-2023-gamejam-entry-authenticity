pub mod components;
mod systems;

use bevy::prelude::*;
use systems::*;
use crate::states::AppState;

pub struct StaticObjectsPlugin;

impl Plugin for StaticObjectsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::InGame), spawn_static_objects);
    }
}