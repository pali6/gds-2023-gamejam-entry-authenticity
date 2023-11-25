use bevy::prelude::*;
use rand::{seq::SliceRandom, Rng};

use crate::{world::WorldParams, states::AppState};

const GRASS_SPRITE_COUNT: usize = 4;

#[derive(Resource, Default)]
struct GrassParms {
    pub grass_count: usize,
    grass_images: Vec<Handle<Image>>,
}

impl GrassParms {
    pub fn get_random_grass_sprite_bundle(&self, pos: Vec2, wiggle: i32) -> SpriteBundle {
        let grass_image = self.grass_images.choose(&mut rand::thread_rng()).unwrap().clone();
        let mut transform = Transform::from_scale(Vec3{
            x: [-1, 1].choose(&mut rand::thread_rng()).unwrap().clone() as f32,
            y: 1.0,
            z: 1.0,
        });
        transform.translation = Vec3::new(
            pos.x + rand::thread_rng().gen_range(-wiggle ..= wiggle) as f32,
            pos.y + rand::thread_rng().gen_range(-wiggle ..= wiggle) as f32,
            -5.0
        );
        SpriteBundle {
            texture: grass_image,
            transform: transform,
            ..Default::default()
        }
    }
}

pub struct GrassPlugin {
    pub grass_count: usize,
}

impl GrassPlugin {
    pub fn new(grass_count: usize) -> Self {
        GrassPlugin { grass_count }
    }
}

impl Plugin for GrassPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GrassParms {
                grass_count: self.grass_count,
                grass_images: Vec::new(),
            })
            .add_systems(Startup, setup_grass)
            .add_systems(OnEnter(AppState::InGame), spawn_grass)
            .add_systems(OnExit(AppState::InGame), despawn_grass);
    }
}

fn setup_grass(
    mut grass_params: ResMut<GrassParms>,
    asset_server: Res<AssetServer>,
) {
    for i in 1..=GRASS_SPRITE_COUNT {
        let grass_image = asset_server.load(format!("sprites/grass-{}.png", i));
        grass_params.grass_images.push(grass_image);
    }
}

#[derive(Component)]
struct Grass;

fn spawn_grass(
    mut commands: Commands,
    grass_params: Res<GrassParms>,
    world_params: Res<WorldParams>,
) {
    let mut positions = Vec::new();
    for _ in 0..grass_params.grass_count {
        positions.push(Vec2::new(
            rand::thread_rng().gen_range(0.0 ..= world_params.width),
            rand::thread_rng().gen_range(0.0 ..= world_params.height),
        ));
    }
    let dist_check = 50.0;
    let min_dist_check = 20.0;
    let max_iter = 10;
    let force_strength = 0.2;
    for iteration in 0..max_iter { 
        let progress_fract = iteration as f32 / max_iter as f32;
        let dist_check = min_dist_check + (dist_check - min_dist_check) * progress_fract;
        for i in 0..grass_params.grass_count {
            let mut repulsive_force = Vec2::ZERO;
            for j in 0..grass_params.grass_count {
                if i == j {
                    continue;
                }
                let diff = positions[i] - positions[j];
                let dist = diff.length();
                if dist < dist_check {
                    repulsive_force += diff.normalize() * (dist_check - dist);
                }
            }
            if positions[i].x < dist_check {
                repulsive_force += Vec2::new(1.0, 0.0) * (dist_check - positions[i].x) * 2.0;
            }
            if positions[i].x > world_params.width - dist_check {
                repulsive_force += Vec2::new(-1.0, 0.0) * (dist_check - (world_params.width - positions[i].x)) * 2.0;
            }
            if positions[i].y < dist_check {
                repulsive_force += Vec2::new(0.0, 1.0) * (dist_check - positions[i].y) * 2.0;
            }
            if positions[i].y > world_params.height - dist_check {
                repulsive_force += Vec2::new(0.0, -1.0) * (dist_check - (world_params.height - positions[i].y)) * 2.0;
            }
            positions[i] += repulsive_force * force_strength;
            /*if positions[i].x < 0.0 || positions[i].x > world_params.width || positions[i].y < 0.0 || positions[i].y > world_params.height {
                positions[i] = Vec2::new(
                    rand::thread_rng().gen_range(0.0 ..= world_params.width),
                    rand::thread_rng().gen_range(0.0 ..= world_params.height),
                );
            }*/
            positions[i].x = positions[i].x.round();
            positions[i].y = positions[i].y.round();
        }
    }
    for _ in 0..grass_params.grass_count {
        commands.spawn((
            grass_params.get_random_grass_sprite_bundle(
                positions.pop().unwrap(),
                10
            ),
            Grass,
        ));
    }
}

fn despawn_grass(
    mut commands: Commands,
    grass_query: Query<Entity, With<Grass>>,
) {
    for entity in grass_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}