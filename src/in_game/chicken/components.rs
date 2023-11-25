use bevy::prelude::Component;

use super::quirk::Quirk;
use super::resources::ChickenParams;

#[derive(Component)]
pub struct Chicken {
    pub name: String,
    pub quirks: Vec<Box<dyn Quirk>>,
    pub movement_speed: f32,
}

impl Chicken {
    pub fn new(name: String, quirks: Vec<Box<dyn Quirk>>) -> Self {
        Self { name, quirks, movement_speed: 500.0 }
    }

    pub fn new_random(chicken_params: &mut ChickenParams) -> Self {
        let name = chicken_params.get_random_name();
        let quirks = vec![];
        Self::new(name, quirks)
    }
}