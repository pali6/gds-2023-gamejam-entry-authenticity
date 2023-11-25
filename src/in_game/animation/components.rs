use bevy::prelude::*;
use rand::seq::index;
use crate::in_game::chicken::components::{BodyPart, ChickenAnimation};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AnimState {
    Idle,
    Running,
    Eating,
    Chilling1,
    Chilling2
}

#[derive(Component)]
pub struct Animation {
    pub frame: usize,
    pub index_buffer: &'static [usize],
    pub time: f32,
    pub period: f32,
    pub repeating: bool,

    body_part: Option<BodyPart>,
    state: AnimState,
    pub is_changed: bool,
}

impl Animation {
    fn get_head_index_buffer(anim_state: AnimState)-> &'static [usize] {
        match anim_state {
            AnimState::Idle => ChickenAnimation::HEAD_IDLE,
            AnimState::Running => ChickenAnimation::HEAD_LOOK_LEFT,
            AnimState::Eating => ChickenAnimation::HEAD_EATING,
            AnimState::Chilling1 => ChickenAnimation::HEAD_ROTATING,
            AnimState::Chilling2 => ChickenAnimation::HEAD_PREENING,
            _ => ChickenAnimation::HEAD_IDLE
        }
    }

    fn get_body_index_buffer(anim_state: AnimState)-> &'static [usize] {
        match anim_state {
            AnimState::Idle => ChickenAnimation::BODY_IDLE,
            AnimState::Running => ChickenAnimation::BODY_RUN,
            AnimState::Eating => ChickenAnimation::BODY_IDLE,
            AnimState::Chilling1 => ChickenAnimation::BODY_IDLE,
            AnimState::Chilling2 => ChickenAnimation::BODY_IDLE,
            _ => ChickenAnimation::BODY_IDLE
        }
    }

    fn get_tail_index_buffer(anim_state: AnimState)-> &'static [usize] {
        match anim_state {
            AnimState::Idle => ChickenAnimation::TAIL_IDLE,
            AnimState::Running => ChickenAnimation::TAIL_WAG,
            AnimState::Eating => ChickenAnimation::TAIL_IDLE,
            AnimState::Chilling1 => ChickenAnimation::TAIL_WAG,
            AnimState::Chilling2 => ChickenAnimation::TAIL_IDLE,
            _ => ChickenAnimation::TAIL_IDLE
        }
    }

    fn get_wing_index_buffer(anim_state: AnimState)-> &'static [usize] {
        match anim_state {
            AnimState::Idle => ChickenAnimation::WING_IDLE,
            AnimState::Running => ChickenAnimation::WING_FLAP,
            AnimState::Eating => ChickenAnimation::WING_IDLE,
            AnimState::Chilling1 => ChickenAnimation::WING_IDLE,
            AnimState::Chilling2 => ChickenAnimation::WING_IDLE,
            _ => ChickenAnimation::HEAD_IDLE
        }
    }

    fn get_index_buffer(body_part: BodyPart, anim_state: AnimState) -> &'static [usize]{
        match body_part {
            BodyPart::Head => Self::get_head_index_buffer(anim_state),
            BodyPart::Body => Self::get_body_index_buffer(anim_state),
            BodyPart::Tail => Self::get_tail_index_buffer(anim_state),
            BodyPart::Wing => Self::get_wing_index_buffer(anim_state),
            _ => ChickenAnimation::HEAD_ROTATING
        }
    }

    pub fn set_state(&mut self, anim_state: AnimState) {
        if self.state == anim_state { return; }
        if let None = self.body_part { return; }

        self.index_buffer = Self::get_index_buffer(self.body_part.unwrap(), anim_state);
        self.is_changed = true;
        self.state = anim_state;
    }

    pub fn new(period: f32, index_buffer: &'static [usize], repeating: bool) -> Self {
        Self {
            frame: Default::default(),
            index_buffer: index_buffer,
            time: 0.0,
            period: period,
            repeating: repeating,
            is_changed: false,
            state: AnimState::Idle, // unused if not chicken
            body_part: None // unused if not chicken
        }
    }

    pub fn new_chicken(period: f32, body_part: BodyPart) -> Self {
        let index_buffer = Self::get_index_buffer(body_part, AnimState::Idle);
        Self {
            frame: Default::default(),
            index_buffer: index_buffer,
            time: 0.0,
            period: period,
            body_part: Some(body_part),
            is_changed: false,
            repeating: true,
            state: AnimState::Idle
        }
    }
}
