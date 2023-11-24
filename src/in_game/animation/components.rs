use bevy::prelude::*;

#[derive(Component)]
pub struct Animation {
    pub index: usize,
    pub current_animation: Vec<Handle<Image>>,
    pub timer: Timer
}

impl Animation {
    pub fn new(period: f32, animation: Vec<Handle<Image>>) -> Self {
        Self {
            index: Default::default(),
            current_animation: Default::default(),
            timer: Timer::from_seconds(period, TimerMode::Repeating)
        }
    }
}