use bevy::prelude::*;
use crate::in_game::chicken::components::{BodyPart, ChickenAnimation};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AnimState {
    Idle,
    Running,
    Eating,
    Chilling_Rotating_Head,
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
            AnimState::Chilling_Rotating_Head => ChickenAnimation::HEAD_ROTATING,
            AnimState::Chilling2 => ChickenAnimation::HEAD_PREENING,
            _ => ChickenAnimation::HEAD_IDLE
        }
    }

    fn get_body_index_buffer(anim_state: AnimState)-> &'static [usize] {
        match anim_state {
            AnimState::Idle => ChickenAnimation::BODY_IDLE,
            AnimState::Running => ChickenAnimation::BODY_RUN,
            AnimState::Eating => ChickenAnimation::BODY_IDLE,
            AnimState::Chilling_Rotating_Head => ChickenAnimation::BODY_IDLE,
            AnimState::Chilling2 => ChickenAnimation::BODY_IDLE,
            _ => ChickenAnimation::BODY_IDLE
        }
    }

    fn get_tail_index_buffer(anim_state: AnimState)-> &'static [usize] {
        match anim_state {
            AnimState::Idle => ChickenAnimation::TAIL_IDLE,
            AnimState::Running => ChickenAnimation::TAIL_WAG,
            AnimState::Eating => ChickenAnimation::TAIL_IDLE,
            AnimState::Chilling_Rotating_Head => ChickenAnimation::TAIL_WAG,
            AnimState::Chilling2 => ChickenAnimation::TAIL_IDLE,
            _ => ChickenAnimation::TAIL_IDLE
        }
    }

    fn get_wing_index_buffer(anim_state: AnimState)-> &'static [usize] {
        match anim_state {
            AnimState::Idle => ChickenAnimation::WING_IDLE,
            AnimState::Running => ChickenAnimation::WING_FLAP,
            AnimState::Eating => ChickenAnimation::WING_IDLE,
            AnimState::Chilling_Rotating_Head => ChickenAnimation::WING_IDLE,
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

pub enum EasingFunction {
    Smooth,
    ElasticOut,
}

impl EasingFunction {
    pub fn ease(&self, x: f32) -> f32 {
        match self {
            Self::Smooth => Self::ease_smooth(x),
            Self::ElasticOut => Self::ease_out_elastic(x)
        }
    }

    pub fn ease_smooth(x: f32) -> f32 {
        -((3.14 * x).cos() - 1.0) / 2.0
    }

    pub fn ease_out_elastic(x: f32) -> f32 {
        let c4 = (2.0 * 3.14) / 3.0;
        
        return if x == 0.0 {
            0.0
        } else if x == 1.0 {
            1.0
        } else {
            2.0_f32.powf(-10.0 * x) * ((x*10.0 - 0.75) * c4).sin() + 1.0
        };
    }
}

#[derive(Component)]
pub struct ScaleTween {
    pub from: Vec3,
    pub to: Vec3,
    pub time: f32,
    pub duration: f32,
    pub easing: EasingFunction
}

impl ScaleTween {
    pub fn new(from: Vec3, to: Vec3, duration: f32, easing: EasingFunction) -> Self {
        Self {
            from: from,
            to: to,
            duration: duration,
            time: 0.0,
            easing: easing
        }
    }
}

#[derive(Component)]
pub struct FadeAwayTween {
    pub delay: f32,
    pub duration: f32,
    pub easing: EasingFunction,
    pub time: f32,
    pub delete_on_completion: bool
}

impl FadeAwayTween {
    pub fn new(delay: f32, duration: f32, easing: EasingFunction, delete_on_completion: bool) -> Self {
        Self {
            delay: delay,
            duration: duration,
            easing: easing,
            time: 0.0,
            delete_on_completion: delete_on_completion
        }
    }
}