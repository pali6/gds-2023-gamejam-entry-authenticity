mod player;

use bevy::prelude::*;
use player::PlayerPlugin;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PlayerPlugin);
    }
}