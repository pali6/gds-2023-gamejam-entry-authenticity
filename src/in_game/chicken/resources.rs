use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;
use std::sync::OnceLock;

static CHICKEN_NAMES_STR: &str = include_str!("chicken_names.txt");

static CHICKEN_NAMES: OnceLock<Vec<&'static str>> = OnceLock::new();

pub struct ChickenVariants;
#[allow(dead_code)]
impl ChickenVariants {
    pub const CHICKEN_VARIANTS: [&'static str; 8] = [
        "sprites/chicken-Sheet-grey.png",
        "sprites/chicken-Sheet-red.png",
        "sprites/chicken-Sheet-blue.png",
        "sprites/chicken-Sheet-brown.png",
        "sprites/chicken-Sheet-purple.png",
        "sprites/chicken-Sheet-orange.png",
        "sprites/chicken-Sheet-rainbow.png",
        "sprites/chicken-Sheet-yellow.png",
    ];
}

#[derive(Resource, Default)]
pub struct ChickenAtlas {
    pub sprite_sheets: Vec<Handle<TextureAtlas>>,
}

#[derive(Resource)]
pub struct ChickenParams {
    unused_names: Vec<&'static str>,
    pub n_foxes_to_spawn: usize,
}

impl Default for ChickenParams {
    fn default() -> Self {
        Self::new()
    }
}

impl ChickenParams {
    pub fn new() -> Self {
        Self {
            unused_names: Vec::new(),
            n_foxes_to_spawn: 0,
        }
    }

    pub fn reset(&mut self) {
        self.unused_names = CHICKEN_NAMES
            .get_or_init(|| {
                CHICKEN_NAMES_STR
                    .lines()
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .collect()
            })
            .clone();
    }

    pub fn get_random_name(&mut self) -> String {
        let mut rng = rand::thread_rng();
        if self.unused_names.len() == 0 {
            self.reset();
        }
        let index = rng.gen_range(0..self.unused_names.len());
        self.unused_names.swap_remove(index).to_string()
    }

    pub fn get_random_chicken_bundle(
        &mut self,
        spawn_x: f32,
        spawn_y: f32,
        asset_server: &Res<AssetServer>,
    ) -> SpriteBundle {
        let texture_name = *[
            "sprites/chicken-1.png",
            "sprites/chicken-2.png",
            "sprites/chicken-3.png",
        ]
        .choose(&mut rand::thread_rng())
        .unwrap();
        SpriteBundle {
            transform: Transform::from_xyz(spawn_x, spawn_y, 0.0),
            texture: asset_server.load(texture_name),
            sprite: Sprite{
                color: Color::rgba(1.0, 1.0, 1.0, 0.0),
                ..default()
            },
            ..default()
        }
    }
}
