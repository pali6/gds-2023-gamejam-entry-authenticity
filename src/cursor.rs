use bevy::{
    prelude::*,
    window::CursorGrabMode,
};

use crate::{states::AppState, in_game::states::InGameState};

pub const SHOOT_CURSOR: &'static str = "sprites/cursor-shoot.png";

pub struct CursorPlugin { }

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, change_window_cursor_icon)
            .add_systems(Startup, setup_cursor)
            .add_systems(Update, move_cursor)
            
            .add_systems(OnEnter(AppState::MainMenu), activate_menu_cusor)
            .add_systems(OnExit(AppState::MainMenu), activate_game_cursor)
            .add_systems(OnEnter(InGameState::Paused), activate_menu_cusor)
            .add_systems(OnExit(InGameState::Paused), activate_game_cursor)
            ;
    }
}

#[derive(Component)]
pub struct GameCursor { }

fn activate_menu_cusor(
    mut windows: Query<&mut Window>,
    mut cursor_query: Query<&mut Visibility, With<GameCursor>>
) {
    let mut window: Mut<Window> = windows.single_mut();
    window.cursor.visible = true;
    if let Ok(mut cursor_visibility) = cursor_query.get_single_mut() {
        *cursor_visibility = Visibility::Hidden;
    }
}

fn activate_game_cursor(
    mut windows: Query<&mut Window>,
    mut cursor_query: Query<&mut Visibility, With<GameCursor>>
) {
    let mut window: Mut<Window> = windows.single_mut();
    window.cursor.visible = false;
    if let Ok(mut cursor_visibility) = cursor_query.get_single_mut() {
        *cursor_visibility = Visibility::Visible;
    }
}

fn setup_cursor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let cursor_spawn: Vec3 = Vec3::ZERO;

    commands.spawn((
        ImageBundle {
            image: asset_server.load(SHOOT_CURSOR).into(),
            style: Style {
                position_type: PositionType::Absolute,
                ..default()
            },
            z_index: ZIndex::Global(15),
            transform: Transform::from_translation(cursor_spawn),
            ..default()
        },
        GameCursor {}
    ));
}

fn move_cursor(
    window: Query<&Window>, 
    mut cursor: Query<&mut Style, With<GameCursor>>
) {
    let window: &Window = window.single();
    if let Some(position) = window.cursor_position() {
        let mut img_style = cursor.single_mut();
        img_style.left = Val::Px(position.x - 16.0);
        img_style.top = Val::Px(position.y - 16.0);
    }
}

//fn toggle_cursor(mut windows: Query<&mut Window>, input: Res<Input<KeyCode>>) {
//    if input.just_pressed(KeyCode::Space) {
//        let mut window = windows.single_mut();
//        window.cursor.grab_mode = match window.cursor.grab_mode {
//            CursorGrabMode::None => CursorGrabMode::Locked,
//            CursorGrabMode::Locked | CursorGrabMode::Confined => CursorGrabMode::None,
//        };
//    }
//}

fn change_window_cursor_icon(
    mut windows: Query<&mut Window>
) {
    let mut window = windows.single_mut();
    window.cursor.icon = CursorIcon::Hand;
}