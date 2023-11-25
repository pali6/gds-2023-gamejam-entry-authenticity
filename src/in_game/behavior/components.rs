use bevy::{prelude::*, ecs::world};
use rand::seq::SliceRandom;

use crate::{utilities::{Dir, get_random_coords_padding}, in_game::{chicken::components::Chicken, animation::{components::Animation, resources::AnimationResource}}, world::WorldParams};

#[derive(Copy, Clone)]
pub enum BehaviorState {
    Moving,
    Waiting,
    Eating,
    Hiding,
}

pub enum BehaviorType {
    RandomMovement,
}

#[derive(Component)]
pub struct Behavior {
    pub state: BehaviorState,
    pub next_state: Option<BehaviorState>,
    pub b_type: BehaviorType,
    pub target: Vec3,
    pub wait_timer: Timer,
    pub wait_duration: f32,
}

impl Behavior {

    pub fn new(b_type: BehaviorType) -> Self {
        Self {
            state: BehaviorState::Waiting,
            next_state: None,
            b_type: b_type,
            target: Vec3::ZERO,
            wait_duration: 4.0,
            wait_timer: Timer::from_seconds(2.0, TimerMode::Once)
        }
    }

    pub fn get_direction(&self, pos: Vec3) -> Option<Dir> {
        let radius: f32 = 50.0;

        if pos.x < (self.target.x - radius) {
            return Some(Dir::Right);
        } else if pos.x > (self.target.x + radius) {
            return Some(Dir::Left);
        } else if pos.y < (self.target.y - radius) {
            return Some(Dir::Up);
        } else if pos.y > (self.target.y + radius) {
            return Some(Dir::Down);
        } else {
            return None;
        }
    }

    pub fn init_waiting(&mut self, duration: f32) {
        self.wait_timer = Timer::from_seconds(duration, TimerMode::Once);
    }

    pub fn init_movement(&mut self, x: f32, y:f32) {
        self.target = Vec3::new(x, y, 0.0);
    }

    pub fn init_eating(&mut self, duration: f32) {
        self.wait_timer = Timer::from_seconds(duration, TimerMode::Once);
    }

    pub fn init_hiding(&mut self, duration: f32) {
        self.wait_timer = Timer::from_seconds(duration, TimerMode::Once);
    }

    pub fn update_waiting(&mut self, time: &Res<Time>, world_params: &Res<WorldParams>) {
        self.wait_timer.tick(time.delta());

        if self.wait_timer.finished() {
            self.state_transition(world_params);
        }
    }

    pub fn update_movement(
        &mut self,
        transform: &mut Transform,
        chicken: &Chicken,
        world_params: &Res<WorldParams>
    ) {

        let dir_maybe = self.get_direction(transform.translation);

        if let Some(dir) = dir_maybe {
            transform.translation += dir.to_vector() * chicken.movement_speed;
        } else {
            self.state_transition(world_params);
        }
    }

    pub fn update_hidnig(&mut self, time: &Res<Time>, world_params: &Res<WorldParams>) {
        self.wait_timer.tick(time.delta());

        if self.wait_timer.finished() {
            self.state_transition(world_params);
        }
    }

    pub fn update_eating(&mut self, time: &Res<Time>, world_params: &Res<WorldParams>) {
        self.wait_timer.tick(time.delta());

        if self.wait_timer.finished() {
            self.state_transition(world_params);
        }
    }

    pub fn to_state(&mut self, state: BehaviorState) {
        match state {
            BehaviorState::Eating => { self.init_eating(self.wait_duration) }
            BehaviorState::Hiding => { self.init_hiding(self.wait_duration) }
            BehaviorState::Waiting => { self.init_waiting(self.wait_duration) }
            _ => {}
        };
    }

    pub fn get_location(state: BehaviorState, world_params: &Res<WorldParams>) -> (f32, f32) {
        match state {
            BehaviorState::Eating => (
                world_params.wheat_location.x, world_params.wheat_location.y
            ),

            BehaviorState::Hiding => (
                world_params.shed_location.x, world_params.shed_location.y
            ),

            BehaviorState::Waiting => {
                get_random_coords_padding(
                    world_params.width, world_params.height,
                    50.0, 50.0)
            },

            _ => {
                get_random_coords_padding(
                    world_params.width, world_params.height,
                    50.0, 50.0)
            }
        }
    }

    pub fn state_transition(&mut self, world_params: &Res<WorldParams>) {

        if let Some(next_state) = self.next_state {
            self.state = next_state;
            self.next_state = None;
            self.to_state(self.state);
            return;
        }

        let states = [
            BehaviorState::Eating,
            BehaviorState::Waiting,
            BehaviorState::Hiding,
        ];
        let next_state = states[rand::random::<usize>() % states.len()];
        self.state = BehaviorState::Moving;
        self.next_state = Some(next_state);
        let (x, y) = Self::get_location(next_state, world_params);
        self.init_movement(x, y);
    }
}