use bevy::prelude::*;

use crate::in_game::states::InGameState;

use super::click::HoveredOverChicken;

pub struct HoverGlowPlugin;

impl Plugin for HoverGlowPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, create_hover_glow)
            .add_systems(PostUpdate, hover_glow);
    }
}

#[derive(Component)]
struct HoverGlow;

fn create_hover_glow(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(-20.0 + 16.0, -20.0 + 16.0, -1.0),
            texture: asset_server.load("sprites/hover_glow.png"),
            ..default()
        },
        HoverGlow,
    ));
}

fn hover_glow(
    hovered_over_chicken: Res<HoveredOverChicken>,
    mut query: Query<(&mut Transform, &mut Sprite, &mut Visibility), With<HoverGlow>>,
    mouse_buttons: Res<Input<MouseButton>>,
    chickens: Query<&Transform, (With<super::components::Chicken>, Without<HoverGlow>)>,
    ingame_state: Res<State<InGameState>>,
) {
    let Ok((mut transform, mut sprite, mut visibility)) = query.get_single_mut() else { return; };

    if *ingame_state != InGameState::Running {
        *visibility = Visibility::Hidden;
        return;
    }

    if let Some(chicken_transform) = hovered_over_chicken.chicken.and_then(|chicken| chickens.get(chicken).ok()) {
        transform.translation = chicken_transform.translation;
        *visibility = Visibility::Visible;
    } else {
        *visibility = Visibility::Hidden;
    }

    if mouse_buttons.pressed(MouseButton::Left) {
        sprite.color = Color::rgba_linear(1.0, 0.15, 0.15, 0.2);
    } else {
        sprite.color = Color::rgba_linear(1.0, 1.0, 1.0, 0.2);
    }
}