use bevy::prelude::*;
use rand::Rng;

use crate::{utilities::{Dir, get_random_coords_padding}, in_game::{chicken::{components::Chicken, quirk::Quirk}, animation::{components::{Animation, ScaleTween, EasingFunction}, resources::AnimationResource}}, world::WorldParams};

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
    pub target: Option<Vec3>,
    pub wait_timer: Timer,
    pub wait_duration: f32,
    pub current_dir: Vec3,
    start: Option<Vec3>,
    path: Vec<Vec3>,
    duration: f32,
    time: f32
}

#[derive(Component)]
pub struct SpeechBubble {
    pub destroy_timer: Timer,
}
impl SpeechBubble {
    pub const THINKING: &'static [usize] = &[8, 9, 10, 11, 0];
    pub const EXTATIC: &'static [usize] = &[8, 9, 10, 11, 12, 1];
    pub const BORED: &'static [usize] = &[8, 9, 10, 11, 12, 2];
    pub const SAD: &'static [usize] = &[8, 9, 10, 11, 12, 3];
    pub const ANGRY: &'static [usize] = &[8, 9, 10, 11, 12, 4];
    pub const SERIOUS: &'static [usize] = &[8, 9, 10, 11, 12, 5];
    pub const EXCLAMATION: &'static [usize] = &[13, 14, 15, 16, 17];
}

impl Behavior {
    fn spawn_speech_bubble(&self, father: Entity, commands: &mut Commands, anim_resource : &Res<AnimationResource>) {
        let mut transform = Transform::from_xyz(0.0, 32.0, 7.0);
        transform.scale = Vec3::new(1.0, 1.0, 1.0);
        let anim_period = 0.09;

        let index_buffer = match self.state {
            BehaviorState::Eating => SpeechBubble::EXTATIC,
            BehaviorState::Hiding => SpeechBubble::SAD,
            BehaviorState::Waiting => SpeechBubble::BORED,
            _ => SpeechBubble::THINKING
        };

        let easing_time: f32 = anim_period * index_buffer.len() as f32 + 0.5;

        let bubble_id = commands.spawn((
            SpeechBubble{ destroy_timer: Timer::from_seconds(self.wait_duration, TimerMode::Once) },
            Animation::new(anim_period, index_buffer, false),
            SpriteSheetBundle {
                texture_atlas: anim_resource.bubble_atlas.clone(),
                sprite: TextureAtlasSprite::new(index_buffer[0]),
                transform: transform,
                ..default()
            },
            ScaleTween::new(Vec3::ZERO, Vec3::ONE, easing_time, EasingFunction::ElasticOut)
        )).id();

        commands.entity(father).add_child(bubble_id);
    }

    pub fn new(b_type: BehaviorType) -> Self {
        Self {
            state: BehaviorState::Waiting,
            next_state: None,
            b_type: b_type,
            target: None,
            start: None,
            wait_duration: 4.0,
            wait_timer: Timer::from_seconds(2.0, TimerMode::Once),
            duration: 0.0,
            time: 0.0,
            path: Vec::new(),
            current_dir: Dir::Left.to_vector()
        }
    }

    pub fn get_direction(&self, pos: Vec3, target: Vec3) -> Option<Dir> {
        let radius: f32 = 20.0;

        if pos.x < (target.x - radius) {
            return Some(Dir::Right);
        } else if pos.x > (target.x + radius) {
            return Some(Dir::Left);
        } else if pos.y < (target.y - radius) {
            return Some(Dir::Up);
        } else if pos.y > (target.y + radius) {
            return Some(Dir::Down);
        } else {
            return None;
        }
    }

    pub fn init_waiting(&mut self, duration: f32) {
        self.wait_timer = Timer::from_seconds(duration, TimerMode::Once);
    }

    pub fn init_movement(&mut self, from: Vec3, to: Vec3, chicken: &Chicken) {
        // L movement
        if chicken.quirk_check(Quirk::NeverGoesDirectly) {
            let horizontal = Vec3::new(to.x, from.y, 0.0);
            self.path.push(horizontal);
        }

        self.path.push(to);
        //let distance = to.x - from.x;
        //self.duration = distance / speed;
        self.time = 0.0;
    }

    pub fn init_eating(&mut self, duration: f32) {
        self.wait_timer = Timer::from_seconds(duration, TimerMode::Once);
    }

    pub fn init_hiding(&mut self, duration: f32) {
        self.wait_timer = Timer::from_seconds(duration, TimerMode::Once);
    }

    pub fn update_waiting(
        &mut self, time: &Res<Time>,
        world_params: &Res<WorldParams>,
        chicken_entity: Entity,
        commands: &mut Commands,
        anim_resource: &Res<AnimationResource>,
        chicken: &Chicken,
        transform: &mut Transform
    ) {
        self.wait_timer.tick(time.delta());

        if self.wait_timer.finished() {
            self.state_transition(world_params, chicken_entity, commands, &anim_resource, chicken, transform);
        }
    }

    pub fn update_movement(
        &mut self,
        transform: &mut Transform,
        chicken: &Chicken,
        world_params: &Res<WorldParams>,
        chicken_entity: Entity,
        commands: &mut Commands,
        anim_resource: &Res<AnimationResource>,
        time: &Res<Time>
    ) {
        if self.target == None && self.path.is_empty() {
            self.state_transition(world_params, chicken_entity, commands, anim_resource, chicken, transform);
            return;
        }

        if self.start == None { self.start = Some(transform.translation) }
        let start = self.start.unwrap();

        if let Some(target) = self.target {
            self.time += time.delta_seconds();
            let t = self.time / self.duration;
            let t_eased = EasingFunction::Smooth.ease(t);

            transform.translation = start + (target - start) * t_eased;

            if t >= 1.0 {
                self.start = self.target;
                self.target = None;
                //self.time = 0.0;
            }

            return;
        }

        if self.target == None {
            self.start = Some(transform.translation);
            let target = self.path.remove(0);
            self.target = Some(target);

            let mut speed = chicken.movement_speed;
            if chicken.quirk_check(Quirk::NeverGoesFast) {
                speed *= 0.3;
            }

            self.duration = target.distance(transform.translation) / speed;
            self.time = 0.0;
            self.current_dir = (target - transform.translation).normalize();
        }
    }

    pub fn update_hidnig(
        &mut self, time: &Res<Time>,
        world_params: &Res<WorldParams>,
        chicken_entity: Entity,
        commands: &mut Commands,
        anim_resource: &Res<AnimationResource>,
        chicken: &Chicken,
        transform: &mut Transform
    ) {
        self.wait_timer.tick(time.delta());

        if self.wait_timer.finished() {
            self.state_transition(world_params, chicken_entity, commands, &anim_resource, chicken,transform);
        }
    }

    pub fn update_eating(
        &mut self, time: &Res<Time>,
        world_params: &Res<WorldParams>,
        chicken_entity: Entity,
        commands: &mut Commands,
        anim_resource: &Res<AnimationResource>,
        chicken: &Chicken,
        transform: &mut Transform
    ) {
        self.wait_timer.tick(time.delta());

        if self.wait_timer.finished() {
            self.state_transition(world_params, chicken_entity, commands, &anim_resource, chicken, transform);
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
                world_params.wheat_location.x + rand::thread_rng().gen_range(-200.0 .. 200.0),
                world_params.wheat_location.y + rand::thread_rng().gen_range(-50.0 .. 50.0)
            ),

            BehaviorState::Hiding => (
                world_params.shed_location.x + rand::thread_rng().gen_range(-40.0 .. 60.0),
                world_params.shed_location.y + rand::thread_rng().gen_range(-60.0 .. 60.0)
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

    pub fn state_transition(
        &mut self, world_params: &Res<WorldParams>,
        chicken_entity: Entity,
        commands: &mut Commands,
        anim_resource: &Res<AnimationResource>,
        chicken: &Chicken,
        transform: &mut Transform
    ) {

        if let Some(next_state) = self.next_state {
            self.state = next_state;
            self.next_state = None;
            self.to_state(self.state);
            self.spawn_speech_bubble(chicken_entity, commands, &anim_resource);
            return;
        }

        let mut states = Vec::new();
        states.push(BehaviorState::Waiting);
        if !chicken.quirk_check(Quirk::NeverEats) { states.push(BehaviorState::Eating); }
        if !chicken.quirk_check(Quirk::NeverSleeps) { states.push(BehaviorState::Hiding); }

        let next_state = states[rand::random::<usize>() % states.len()];
        self.state = BehaviorState::Moving;
        self.next_state = Some(next_state);
        let (x, y) = Self::get_location(next_state, world_params);
        self.init_movement(transform.translation, Vec3::new(x, y, 0.0), chicken);
    }
}