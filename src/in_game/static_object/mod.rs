pub mod components;
mod systems;
mod resources;

use bevy::prelude::*;
use systems::*;
use crate::states::AppState;

pub struct StaticObjectsPlugin;

impl Plugin for StaticObjectsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<resources::StaticObjectsResource>()
            .add_systems(Startup, load_static_object_assets)
            .add_systems(OnEnter(AppState::InGame), spawn_static_objects)
            .add_systems(OnExit(AppState::InGame), despawn_static_objects);
    }
}