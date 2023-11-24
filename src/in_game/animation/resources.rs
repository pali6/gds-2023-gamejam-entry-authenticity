use bevy::prelude::*;
use crate::utilities::Directions;

#[derive(Resource)]
pub struct AnimationResource {
    pub frame_period: f32,
    pub hen_idle: Vec<Handle<Image>>,
    pub hen_walking_left: Vec<Handle<Image>>,
    pub hen_walking_right: Vec<Handle<Image>>,
    pub hen_walking_up: Vec<Handle<Image>>,
    pub hen_walking_down: Vec<Handle<Image>>, 
}

impl AnimationResource {
    pub fn get_hen_walking(&self, direction: Option<Directions>) -> Vec<Handle<Image>> {
        let walking_direction = match direction {
            Some(dir) => dir,
            None => return self.hen_idle.clone()
        };

        return match walking_direction {
            Directions::Left => self.hen_walking_left.clone(),
            Directions::Right => self.hen_walking_right.clone(),
            Directions::Up => self.hen_walking_up.clone(),
            Directions::Down => self.hen_walking_down.clone(),
        }
    }
}

impl Default for AnimationResource {
    fn default() -> Self {
        AnimationResource {
            frame_period: 0.5,
            hen_idle: Vec::new(),
            hen_walking_left: Vec::new(),
            hen_walking_right: Vec::new(),
            hen_walking_up: Vec::new(),
            hen_walking_down: Vec::new(),
        }
    }
}