// Just some useful functions you might want to use.
// TODO: Add some more? Move them somewhere else?
//===================================================
use bevy::prelude::*;
use rand::{seq::SliceRandom, *};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Dir {
    pub fn to_vector(&self) -> Vec3 {
        match self {
            Dir::Left => DirectionVectors::LEFT,
            Dir::Right => DirectionVectors::RIGHT,
            Dir::Up => DirectionVectors::UP,
            Dir::Down => DirectionVectors::DOWN,
        }
    }

    pub fn from_vector(vector: Vec3) -> Option<Self> {
        if vector.length() == 0.0 {
            return None;
        }
        if vector.x.abs() > vector.y.abs() {
            if vector.x > 0.0 {
                Some(Dir::Right)
            } else {
                Some(Dir::Left)
            }
        } else {
            if vector.y > 0.0 {
                Some(Dir::Up)
            } else {
                Some(Dir::Down)
            }
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
        }
    }

    pub fn random() -> Self {
        *[Dir::Left, Dir::Right, Dir::Up, Dir::Down]
            .choose(&mut rand::thread_rng())
            .unwrap()
    }
}

impl From<Dir> for Vec3 {
    fn from(direction: Dir) -> Self {
        direction.to_vector()
    }
}

pub struct DirectionVectors;
impl DirectionVectors {
    const UP: Vec3 = Vec3::Y;
    const LEFT: Vec3 = Vec3::NEG_X;
    const DOWN: Vec3 = Vec3::NEG_Y;
    const RIGHT: Vec3 = Vec3::X;
}

// Only takes `.ogg` files, as to my knowledge
pub fn play_sfx(name: String, commands: &mut Commands, asset_server: &Res<AssetServer>, volume: f32) {
    let sfx = asset_server.load(name);
    commands.spawn(AudioBundle {
        source: sfx,
        settings: PlaybackSettings::DESPAWN.with_volume(bevy::audio::Volume::new_relative(volume)),
        ..default()
    });
}

pub fn get_random_coords_range(min_x: f32, max_x: f32, min_y: f32, max_y: f32) -> (f32, f32) {
    let width = max_x - min_x;
    let height = max_y - min_y;
    let x = random::<f32>() * width + min_x;
    let y = random::<f32>() * height + min_y;
    return (x, y);
}

pub fn get_random_coords_padding(width:f32, height: f32, padding_x: f32, padding_y: f32) -> (f32, f32) {
    let min_x = padding_x;
    let min_y = padding_y;
    let max_x = width - padding_x * 2.0;
    let max_y = height - padding_y * 2.0;
    return get_random_coords_range(min_x, max_x, min_y, max_y);
}

pub fn get_random_coords(width: f32, height: f32) -> (f32, f32) {
    get_random_coords_padding(width, height, 0.0, 0.0)
}

pub fn get_direction(keyboard_input: Res<Input<KeyCode>>) -> Vec3 {
    let mut direction = Vec3::ZERO;
    let is_pressed = |code: KeyCode| -> bool { keyboard_input.pressed(code) };
    use KeyCode::*;

    if is_pressed(Left) || is_pressed(A) {
        direction += DirectionVectors::LEFT;
    }

    if is_pressed(Right) || is_pressed(D) {
        direction += DirectionVectors::RIGHT;
    }

    if is_pressed(Up) || is_pressed(W) {
        direction += DirectionVectors::UP;
    }

    if is_pressed(Down) || is_pressed(S) {
        direction += DirectionVectors::DOWN;
    }

    if direction.length() > 0.0 {
        direction = direction.normalize();
    }

    return direction;
}

pub fn confine_movement(
    mut x: f32,
    mut y: f32,
    width: f32,
    height: f32,
    object_width: f32,
    object_height: f32,
) -> (f32, f32) {
    let half_width = object_width / 2.0;
    let half_height = object_height / 2.0;
    let x_min = 0.0 + half_width;
    let y_min = 0.0 + half_height;
    let x_max = width - half_width;
    let y_max = height - half_height;

    if x < x_min {
        x = x_min;
    }
    if y < y_min {
        y = y_min;
    }
    if x > x_max {
        x = x_max;
    }
    if y > y_max {
        y = y_max;
    }

    return (x, y);
}
