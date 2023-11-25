use bevy::prelude::*;

#[derive(Resource)]
pub struct WorldParams {
    pub width: f32,
    pub height: f32,
    pub shed_location: Vec3,
    pub wheat_location: Vec3,
    pub quirks_per_chicken: usize,
    pub chicken_count: usize,
    pub fox_count: usize,
}
