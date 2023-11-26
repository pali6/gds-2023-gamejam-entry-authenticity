use bevy::prelude::*;
use itertools::Itertools;
use rand::seq::SliceRandom;

use crate::{in_game::{states::InGameState, delayed_death_count::{DelayedDeathCount, queue_delayed_death_count, DeathType}, inworld_object::InWorldObject}, states::AppState, utilities::play_sfx};

use super::{components::Chicken, kill_click::DeadChicken};

pub struct ChickenHuntPlugin;

impl Plugin for ChickenHuntPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ChickenHunt>()
            .add_systems(OnEnter(AppState::InGame), start_chicken_hunt)
            .add_systems(OnExit(InGameState::Running), pause_chicken_hunt)
            .add_systems(OnEnter(InGameState::Running), resume_chicken_hunt)
            .add_systems(Update, process_chicken_hunt
                .run_if(in_state(AppState::InGame).and_then(in_state(InGameState::Running)))
            );
    }
}

#[derive(Resource, Default)]
pub struct ChickenHunt {
    pub hunt_timer: Timer,
}

fn start_chicken_hunt(
    mut hunt: ResMut<ChickenHunt>,
    world_params: ResMut<crate::world::WorldParams>,
) {
    hunt.hunt_timer = Timer::from_seconds(world_params.chicken_murder_interval, TimerMode::Repeating);
}

fn pause_chicken_hunt() {}

fn resume_chicken_hunt() {}

fn process_chicken_hunt(
    mut commands: Commands,
    mut hunt: ResMut<ChickenHunt>,
    chickens: Query<(Entity, &mut Chicken, &Transform)>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut queued_death_count: ResMut<DelayedDeathCount>,
    world_params: ResMut<crate::world::WorldParams>
) {
    hunt.hunt_timer.tick(time.delta());
    if hunt.hunt_timer.just_finished() {
        let non_fox_chickens = chickens.iter().filter(|(_, chicken, _)| !chicken.is_fox).collect_vec();
        if non_fox_chickens.is_empty() || world_params.foxes_alive <= 0 {
            return;
        }
        let (entity, _, transform) = non_fox_chickens.choose(&mut rand::thread_rng()).unwrap();

        play_sfx("sounds/death_bwok.ogg".to_string(), &mut commands, &asset_server, 1.0);
        let pos = transform.translation;
        commands.entity(*entity).despawn_recursive();

        commands.spawn((
            SpriteBundle{
                texture: asset_server.load("sprites/chicken-killed.png"),
                transform: Transform::from_xyz(pos.x, pos.y, pos.z),
                ..default()
            },
            DeadChicken,
            InWorldObject,
        ));

        queue_delayed_death_count(0.0, DeathType::Chicken, &mut queued_death_count);
    }
}