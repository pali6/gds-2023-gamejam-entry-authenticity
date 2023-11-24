// Just some useful functions you might want to use.
// TODO: Add some more? Move them somewhere else?
//===================================================
use bevy::prelude::*;
use rand::*;

struct Directions;
impl Directions {
    const UP: Vec3 = Vec3::Y;
    const LEFT: Vec3 = Vec3::NEG_X;
    const DOWN: Vec3 = Vec3::NEG_Y;
    const RIGHT: Vec3 = Vec3::X;
}

// Only takes `.ogg` files, as to my knowledge
pub fn play_sfx(name: &'static str, commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let sfx = asset_server.load(name);
    commands.spawn(AudioBundle {
        source: sfx,
        settings: PlaybackSettings::DESPAWN,
        ..default()
    });
}

pub fn get_random_coords(width: f32, height: f32) -> (f32, f32) {
    let x = random::<f32>() * width;
    let y = random::<f32>() * height;
    return (x, y);
}

pub fn get_direction(keyboard_input: Res<Input<KeyCode>>) -> Vec3 {
    let mut direction = Vec3::ZERO;
    let is_pressed = |code: KeyCode| -> bool { keyboard_input.pressed(code) };
    use KeyCode::*;

    if is_pressed(Left) || is_pressed(A) {
        direction += Directions::LEFT;
    }

    if is_pressed(Right) || is_pressed(D) {
        direction += Directions::RIGHT;
    }

    if is_pressed(Up) || is_pressed(W) {
        direction += Directions::UP;
    }

    if is_pressed(Down) || is_pressed(S) {
        direction += Directions::DOWN;
    }

    if direction.length() > 0.0 {
        direction = direction.normalize();
    }

    return direction;
}

pub fn confine_movement(mut x: f32, mut y: f32, width: f32, height: f32, size: f32) -> (f32, f32) {
    let half_size = size / 2.0;
    let x_min = 0.0 + half_size;
    let y_min = 0.0 + half_size;
    let x_max = width - half_size;
    let y_max = height - half_size;

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
