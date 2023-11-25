use bevy::prelude::Component;
use bevy::sprite::Sprite;

use super::quirk::Quirk;
use super::resources::ChickenParams;

pub enum BodyPart {
    Head,
    Wing,
    Tail,
    Body,
}

pub struct ChickenAnimation {
}

impl ChickenAnimation {
    pub fn body_all()           -> Vec<i32> { vec![ 2, 3, 4, 5, 6, 7 ] }
    pub fn wing_all()           -> Vec<i32> { vec![ 8, 9, 10 ] }
    pub fn tail_all()           -> Vec<i32> { vec![ 11, 12, 13 ] }
    pub fn head_all()           -> Vec<i32> { vec![ 14, 15, 16, 17, 18, 19 ] } 
    pub fn body_run()           -> Vec<i32> { vec![ 3, 4, 5, 6] }
    pub fn body_idle()          -> Vec<i32> { vec![ 2 ] }
    pub fn body_sit()           -> Vec<i32> { vec![ 7 ] }
    pub fn wing_flap()          -> Vec<i32> { vec![ 8, 9, 10 ] }
    pub fn tail_wag()           -> Vec<i32> { vec![ 11, 12, 13 ] }
    pub fn head_preening()      -> Vec<i32> { vec![ 18 ] }
    pub fn head_eating()        -> Vec<i32> { vec![ 19 ] }
    pub fn head_look_left()     -> Vec<i32> { vec![ 14 ] }
    pub fn head_look_away()     -> Vec<i32> { vec![ 16 ] }
    pub fn head_look_forward()  -> Vec<i32> { vec![ 15 ] }
    pub fn head_look_right()    -> Vec<i32> { vec![ 17 ] }
    pub fn head_rotating()      -> Vec<i32> { vec![ 14, 15, 17, 16 ] }
}

#[derive(Component)]
pub struct ChickenPart {
    pub part: BodyPart,
    pub sprite: Sprite,
}

#[derive(Component)]
pub struct Chicken {
    pub name: String,
    pub quirks: Vec<Box<dyn Quirk>>,
    pub movement_speed: f32,
}

impl Chicken {
    pub fn new(name: String, quirks: Vec<Box<dyn Quirk>>) -> Self {
        Self {
            name,
            quirks,
            movement_speed: 50.0,
        }
    }

    pub fn new_random(chicken_params: &mut ChickenParams) -> Self {
        let name = chicken_params.get_random_name();
        let quirks = vec![];
        Self::new(name, quirks)
    }
}
