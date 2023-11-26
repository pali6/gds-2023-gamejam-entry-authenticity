
use bevy::prelude::*;

pub struct AmbiencePlugin;

impl Plugin for AmbiencePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_ambience);
    }
}

#[derive(Component)]
pub struct AmbiencePlayer;

fn spawn_ambience(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let sfx = asset_server.load("sounds/ambience.ogg");
    commands.spawn((
        AmbiencePlayer,
        AudioBundle {
            source: sfx,
            settings: PlaybackSettings::LOOP.with_volume(bevy::audio::Volume::new_relative(1.0)),
            ..default()
        }
    ));
}

#[allow(dead_code)]
pub fn stop_ambience(
    mut ambience_players: Query<&mut AudioSink, With<AmbiencePlayer>>,
) {
    for sink in ambience_players.iter_mut() {
        sink.pause();
    }
}

#[allow(dead_code)]
pub fn start_ambience(
    mut ambience_players: Query<&mut AudioSink, With<AmbiencePlayer>>,
) {
    for sink in ambience_players.iter_mut() {
        sink.play();
    }
}

#[allow(dead_code)]
pub fn toggle_ambience(
    mut ambience_players: Query<&mut AudioSink, With<AmbiencePlayer>>,
) {
    for sink in ambience_players.iter_mut() {
        if sink.is_paused() {
            sink.play();
        } else {
            sink.pause();
        }
    }
}