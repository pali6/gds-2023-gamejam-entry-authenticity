use bevy::prelude::*;

#[derive(Resource)]
pub struct StaticObjectsResource {
    pub shed_location: Vec3,
    pub wheat_location: Vec3,
}

impl Default for StaticObjectsResource {
    fn default() -> Self {
        Self {
            shed_location: Vec3::new(500.0, 800.0, 0.0),
            wheat_location: Vec3::new(500.0, 400.0, 0.0),
        }
    }
}