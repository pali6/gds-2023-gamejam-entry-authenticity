pub mod components;
pub mod resources;
mod systems;

use bevy::prelude::*;
use resources::*;
use systems::*;

use crate::states::AppState;

use super::states::InGameState;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AnimationResource>()
            .add_systems(Startup, load_animation_resources)
            .add_systems(
                Update,
                (update_animations, update_scale_tween, )
                    .run_if(in_state(AppState::InGame).and_then(in_state(InGameState::Running))),
            );
    }
}
