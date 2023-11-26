
use bevy::prelude::*;

use crate::utilities::play_sfx;

pub struct TimedSoundsPlugin;

impl Plugin for TimedSoundsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<TimedSounds>()
            .add_systems(Update, play_timed_sounds);
    }
}

#[derive(Resource, Default)]
pub struct TimedSounds {
    pub sounds: Vec<(Timer, String, f32)>,
}

pub fn play_timed_sounds(
    mut commands: Commands,
    time: Res<Time>,
    mut sounds: ResMut<TimedSounds>,
    asset_server: Res<AssetServer>,
) {
    for (timer, sound, volume) in sounds.sounds.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            play_sfx(sound.to_string(), &mut commands, &asset_server, *volume)
        }
    }
    sounds.sounds.retain(|(timer, _, _)| !timer.finished());
}

pub fn play_sound_in(seconds: f32, sound: &str, volume: f32, sounds: &mut TimedSounds) {
    sounds.sounds.push((Timer::from_seconds(seconds, TimerMode::Once), sound.to_string(), volume));
}