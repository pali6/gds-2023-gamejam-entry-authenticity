
use bevy::prelude::*;
pub struct DelayedDeathCountPlugin;
use crate::AppState;

impl Plugin for DelayedDeathCountPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DelayedDeathCount>()
            .add_systems(Update, process_delayed_death_count)
            .add_systems(OnEnter(AppState::InGame), reset_delayed_death_count);
    }
}

pub enum DeathType {
    Fox,
    Chicken,
}

#[derive(Resource, Default)]
pub struct DelayedDeathCount {
    pub queued: Vec<(Timer, DeathType)>,
}

fn process_delayed_death_count(
    time: Res<Time>,
    mut queued: ResMut<DelayedDeathCount>,
    mut world_params: ResMut<crate::world::WorldParams>,
) {
    for (timer, death_type) in queued.queued.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            match death_type {
                DeathType::Fox => world_params.foxes_alive -= 1,
                DeathType::Chicken => world_params.chicken_alive -= 1,
            }
        }
    }
    queued.queued.retain(|(timer, _)| !timer.finished());
}

fn reset_delayed_death_count(mut queued: ResMut<DelayedDeathCount>) {
    queued.queued.clear();
}

pub fn queue_delayed_death_count(seconds: f32, death_type: DeathType, queued: &mut ResMut<DelayedDeathCount>) {
    queued.queued.push((Timer::from_seconds(seconds, TimerMode::Once), death_type));
}