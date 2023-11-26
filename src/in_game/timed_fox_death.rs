
use bevy::prelude::*;

use super::{chicken::kill_click::DeadChicken, animation::components::{FadeAwayTween, EasingFunction}};

pub struct TimedFoxDeathPlugin;

impl Plugin for TimedFoxDeathPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<TimedFoxDeath>()
            .add_systems(Update, process_timed_fox_deaths);
    }
}

#[derive(Resource, Default)]
pub struct TimedFoxDeath {
    pub queued: Vec<(Timer, Entity)>,
}

pub fn process_timed_fox_deaths(
    mut commands: Commands,
    time: Res<Time>,
    mut queued: ResMut<TimedFoxDeath>,
    asset_server: Res<AssetServer>,
    mut dead_chickens: Query<&mut Handle<Image>, With<DeadChicken>>,
) {
    for (timer, entity) in queued.queued.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            if let Ok(mut image) = dead_chickens.get_mut(*entity) {
                *image = asset_server.load("sprites/fox-dead-pali-black.png");
                commands.entity(*entity).insert(FadeAwayTween::new(0.3, 1.0, EasingFunction::Smooth, true));
            }
        }
    }
    queued.queued.retain(|(timer, _)| !timer.finished());
}

pub fn queue_timed_fox(seconds: f32, entity: Entity, queued: &mut ResMut<TimedFoxDeath>) {
    queued.queued.push((Timer::from_seconds(seconds, TimerMode::Once), entity));
}