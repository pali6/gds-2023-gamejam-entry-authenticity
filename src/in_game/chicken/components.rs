use bevy::asset::{Handle, AssetServer, Assets};
use bevy::ecs::system::{ResMut, Res};
use bevy::math::Vec2;
use bevy::prelude::{Component, default};
use bevy::sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite};
use bevy::transform::components::Transform;
use rand::Rng;
use rand::seq::SliceRandom;

use crate::world::WorldParams;

use super::quirk::{Quirk, annotate_quirks, get_n_random_quirks};
use super::resources::{ChickenParams, ChickenAtlas, ChickenVariants};

#[derive(Copy, Clone)]
pub enum BodyPart {
    Head,
    Wing,
    Tail,
    Body
}

pub struct ChickenAnimation;
#[allow(dead_code)]
// for best visuals Head > Wing > Tail > Body
impl ChickenAnimation {
    // All options [do not use these]
    pub const BODY_ALL: &'static [usize] = &[1, 2, 3, 4, 5, 6];
    pub const TAIL_ALL: &'static [usize] = &[10, 11, 12];
    pub const WING_ALL: &'static [usize] = &[7, 8, 9];
    pub const HEAD_ALL: &'static [usize] = &[13, 14, 15, 16, 17, 18];
    // All valid options for idle
    pub const BODY_IDLE: &'static [usize] = &[1];
    pub const TAIL_IDLE: &'static [usize] = &[10];
    pub const WING_IDLE: &'static [usize] = &[7];
    pub const HEAD_IDLE: &'static [usize] = &[13];
    // Body animation
    pub const BODY_RUN: &'static [usize] = &[2, 3, 4, 5];
    pub const BODY_SIT: &'static [usize] = &[6];
    // Tail animation
    pub const TAIL_WAG: &'static [usize] = &[10, 11, 12];
    // Wing animation
    pub const WING_FLAP: &'static [usize] = &[7, 8, 9];
    // Head animation
    pub const HEAD_PREENING: &'static [usize] = &[17];
    pub const HEAD_EATING: &'static [usize] = &[18];
    pub const HEAD_LOOK_LEFT: &'static [usize] = &[13];
    pub const HEAD_LOOK_AWAY: &'static [usize] = &[15];
    pub const HEAD_LOOK_FORWARD: &'static [usize] = &[14];
    pub const HEAD_LOOK_RIGHT: &'static [usize] = &[16];
    pub const HEAD_ROTATING: &'static [usize] = &[13, 14, 16, 15];
}

pub struct ChickenParts {
    pub head: SpriteSheetBundle,
    pub wing: SpriteSheetBundle,
    pub tail: SpriteSheetBundle,
    pub body: SpriteSheetBundle,
}

#[derive(Component)]
pub struct ChickenPart;

impl ChickenParts {
    pub fn new_idle(texture_atlas_handle: Handle<TextureAtlas>) -> Self {
        Self {
            body: ChickenParts::new_sprite_from_array(texture_atlas_handle.clone(), ChickenAnimation::BODY_IDLE, 1.0), 
            tail: ChickenParts::new_sprite_from_array(texture_atlas_handle.clone(), ChickenAnimation::TAIL_IDLE, 2.0), 
            wing: ChickenParts::new_sprite_from_array(texture_atlas_handle.clone(), ChickenAnimation::WING_IDLE, 3.0),  
            head: ChickenParts::new_sprite_from_array(texture_atlas_handle.clone(), ChickenAnimation::HEAD_IDLE, 4.0), 
        }
    }

    pub fn new_sprite_from_array(texture_atlas_handle: Handle<TextureAtlas>, indices: &[usize], z_index: f32) -> SpriteSheetBundle {
        let mut rng = rand::thread_rng();
        let index = *indices.choose(&mut rng).unwrap();
        ChickenParts::new_sprite_single(texture_atlas_handle, index, z_index)
    }

    pub fn new_sprite_single(texture_atlas_handle: Handle<TextureAtlas>, index: usize, z_index: f32) -> SpriteSheetBundle {
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(index),
            transform: Transform::from_xyz(0.0, 0.0, z_index),
            ..default()
        }
    }

    pub fn add_chicken_parts_to_atlas(
        mut chicken_atlas: ResMut<ChickenAtlas>,
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>
    ) {
        for path in ChickenVariants::CHICKEN_VARIANTS {
            let texture_handle = asset_server.load(path);
            let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 19, 1, Some(Vec2 { x: 1.0, y: 0.0 }), Some(Vec2{x: 0.5, y: 0.5}));
            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            chicken_atlas.sprite_sheets.push(texture_atlas_handle);
        }
    }
}

#[derive(Component)]
pub struct Chicken {
    pub name: String,
    pub quirks: Vec<(Quirk, String)>,
    pub movement_speed: f32,
    pub is_fox: bool, // sussy impostor
}

impl Chicken {
    pub fn new(name: String, quirks: Vec<Quirk>) -> Self {
        Self {
            name,
            quirks: annotate_quirks(quirks),
            movement_speed: 200.0,
            is_fox: false,
        }
    }

    pub fn new_random(chicken_params: &mut ChickenParams, n_quirks: usize) -> Self {
        let name = chicken_params.get_random_name();
        let quirks = get_n_random_quirks(n_quirks);
        Self::new(name, quirks)
    }

    pub fn quirk_check(&self, quirk: Quirk, world_params: &WorldParams) -> bool {
        if !self.quirks.iter().any(|(q, _)| *q == quirk) {
            return false;
        }
        if !self.is_fox {
            return true;
        }
        let mut rng = rand::thread_rng();
        let roll = rng.gen_range(0.0..1.0);
        if roll < world_params.quirk_deception_chance {
            return true;
        }
        false
    }
}