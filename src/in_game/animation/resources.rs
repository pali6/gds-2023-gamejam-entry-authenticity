use crate::utilities::Dir;
use bevy::prelude::*;

#[derive(Resource)]
pub struct AnimationResource {
    pub frame_period: f32,
    pub hen_idle: Vec<Handle<Image>>,
    pub hen_walking_left: Vec<Handle<Image>>,
    pub hen_walking_right: Vec<Handle<Image>>,
    pub hen_walking_up: Vec<Handle<Image>>,
    pub hen_walking_down: Vec<Handle<Image>>,
    pub rotating_pizza: Vec<Handle<Image>>,
    pub bubble_atlas: Handle<TextureAtlas>
}

impl AnimationResource {
    pub fn get_hen_walking(&self, direction: Option<Dir>) -> Vec<Handle<Image>> {
        let walking_direction = match direction {
            Some(dir) => dir,
            None => return self.hen_idle.clone(),
        };

        return match walking_direction {
            Dir::Left => self.hen_walking_left.clone(),
            Dir::Right => self.hen_walking_right.clone(),
            Dir::Up => self.hen_walking_up.clone(),
            Dir::Down => self.hen_walking_down.clone(),
        };
    }
}

impl Default for AnimationResource {
    fn default() -> Self {
        AnimationResource {
            frame_period: 0.7,
            hen_idle: Vec::new(),
            hen_walking_left: Vec::new(),
            hen_walking_right: Vec::new(),
            hen_walking_up: Vec::new(),
            hen_walking_down: Vec::new(),
            rotating_pizza: Vec::new(),
            bubble_atlas: Default::default()
        }
    }
}
