use bevy::prelude::*;
use rand::Rng;
use rand::seq::SliceRandom;
use std::sync::OnceLock;

static CHICKEN_NAMES_STR: &str = include_str!("chicken_names.txt");

static CHICKEN_NAMES: OnceLock<Vec<&'static str>> = OnceLock::new();

#[derive(Resource)]
pub struct ChickenParams {
    unused_names: Vec<&'static str>,
}

impl Default for ChickenParams {
    fn default() -> Self {
        Self::new()
    }
}

impl ChickenParams {
    pub fn new() -> Self {
        Self { unused_names: Vec::new() }
    }

    pub fn reset(&mut self) {
        self.unused_names = CHICKEN_NAMES.get_or_init(|| {
            CHICKEN_NAMES_STR
                .lines()
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect()
        }).clone();
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
        let texture_name = *["sprites/chicken-1.png", "sprites/chicken-2.png", "sprites/chicken-3.png"]
            .choose(&mut rand::thread_rng())
            .unwrap();
        SpriteBundle { // TODO: unique random sprites
            transform: Transform::from_xyz(spawn_x, spawn_y, 0.0),
            texture: asset_server.load(texture_name),
            ..default()
        }
    }
}