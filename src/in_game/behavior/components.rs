use bevy::{prelude::*, ecs::{component::TableStorage}};

use crate::{utilities::{Dir, get_random_coords_padding}, in_game::{chicken::components::Chicken, animation::{components::Animation, resources::AnimationResource}}, world::WorldParams};

pub enum BehaviorState {
    Moving,
    Waiting,
}

pub enum BehaviorType {
    RandomMovement,
}

#[derive(Component)]
pub struct Behavior {
    pub state: BehaviorState,
    pub b_type: BehaviorType,
    pub target: Vec3,
    pub wait_timer: Timer,
    pub wait_duration: f32
}

impl Behavior {

    pub fn new(b_type: BehaviorType) -> Self {
        Self {
            state: BehaviorState::Waiting,
            b_type: b_type,
            target: Vec3::ZERO,
            wait_duration: 2.0,
            wait_timer: Timer::from_seconds(5.0, TimerMode::Once)
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

    pub fn movement_init(&mut self, x: f32, y:f32) {
        self.target = Vec3::new(x, y, 0.0);
    }

    // PlaceHolder
    pub fn movement_init_random(&mut self, world_params: &Res<WorldParams>) {
        let (x, y) = get_random_coords_padding(
            world_params.width, world_params.height,
            50.0, 50.0);

        self.movement_init(x, y);
    }

    pub fn update_movement(
        &mut self,
        transform: &mut Transform,
        animation: &mut Animation,
        chicken: &Chicken
    ) {

        let dir_maybe = self.get_direction(transform.translation);
        //animation.current_animation = animation_resource.get_hen_walking(dir_maybe);

        if let Some(dir) = dir_maybe {
            transform.translation += dir.to_vector() * chicken.movement_speed;
        } else {
            self.state = BehaviorState::Waiting;
            self.init_waiting(self.wait_duration);
        }
    }

    pub fn init_waiting(&mut self, duration: f32) {
        self.wait_timer = Timer::from_seconds(duration, TimerMode::Once);
    }

    pub fn update_waiting(&mut self, time: &Res<Time>, world_params: &Res<WorldParams>) {
        self.wait_timer.tick(time.delta());

        if self.wait_timer.finished() {
            self.movement_init_random(world_params);
            self.state = BehaviorState::Moving;
        }
    }
}