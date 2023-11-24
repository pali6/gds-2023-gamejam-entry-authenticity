use bevy::prelude::*;

// Simple resource
// Can be accessed in a system almost like a singleton
#[derive(Resource)]
pub struct ExamplePlayerResource {}

impl Default for ExamplePlayerResource {
    fn default() -> Self {
        ExamplePlayerResource {}
    }
}
