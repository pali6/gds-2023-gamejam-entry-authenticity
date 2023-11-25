use std::collections::HashMap;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContexts};

use crate::states::AppState;

use super::components::Chicken;
use super::click::ChickenClickEvent;

pub struct InfoMenuPlugin;

impl Plugin for InfoMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ChickenMenus>()
            .add_systems(OnEnter(AppState::InGame), reset_chicken_menus)
            .add_systems(Update, (open_chicken_menu, display_menus));
    }
}

#[derive(Debug)]
struct WindowState {
    open: bool,
    just_opened: bool,
}

#[derive(Resource, Default, Debug)]
struct ChickenMenus {
    pub chickens: HashMap<Entity, WindowState>,
}

fn reset_chicken_menus(mut menus: ResMut<ChickenMenus>) {
    menus.chickens.clear();
}

fn open_chicken_menu(
    mut events: EventReader<ChickenClickEvent>,
    mut menus: ResMut<ChickenMenus>,
) {
    for event in events.read() {
        if event.mouse_button.just_pressed(MouseButton::Right) {
            let entity = event.chicken;
            if let Some(WindowState{open, just_opened}) = menus.chickens.get_mut(&entity) {
                *open = !*open;
                if *open {
                    *just_opened = true;
                }
            } else {
                menus.chickens.insert(entity, WindowState{open: true, just_opened: true});
            }
        }
    }
}

fn display_menus(
    mut menus: ResMut<ChickenMenus>,
    query: Query<&Chicken>,
    mut egui_contexts: EguiContexts,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let ctx = egui_contexts.ctx_mut();
    let mut style = (*ctx.style()).clone();
    style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgba_unmultiplied(200, 200, 200, 200);
    style.visuals.widgets.noninteractive.bg_stroke.color = egui::Color32::from_rgba_unmultiplied(0, 0, 0, 230);
    style.visuals.widgets.noninteractive.fg_stroke.color = egui::Color32::from_rgba_unmultiplied(0, 0, 0, 230);
    style.visuals.widgets.active.fg_stroke.color = egui::Color32::from_rgba_unmultiplied(80, 80, 80, 230);
    style.visuals.widgets.hovered.fg_stroke.color = egui::Color32::from_rgba_unmultiplied(30, 30, 30, 230);
    style.visuals.widgets.inactive.fg_stroke.color = egui::Color32::from_rgba_unmultiplied(0, 0, 0, 230);
    style.spacing.icon_width = 20.0;
    style.text_styles.insert(
        egui::TextStyle::Body,
        egui::FontId::new(20.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Heading,
        egui::FontId::new(30.0, egui::FontFamily::Proportional),
    );
    ctx.set_style(style);

    let mouse_position = windows.get_single().ok().and_then(Window::cursor_position);

    for (entity, WindowState {open, just_opened} ) in menus.chickens.iter_mut() {
        if let Ok(chicken) = query.get(*entity) {
            let frame = egui::Frame::default()
                .rounding(5.0)
                .outer_margin(2.0)
                .fill(egui::Color32::from_rgba_unmultiplied(200, 200, 200, 180));
            let mut window = egui::Window::new(&chicken.name)
                .resizable(false)
                .collapsible(true)
                .open(open)
                .frame(frame);
            if *just_opened {
                if let Some(mouse_position) = mouse_position {
                    window = window.current_pos(egui::Pos2::new(mouse_position.x, mouse_position.y));
                }
            }
            if let Some(inner_response) = window.show(ctx, |ui| {
                ui.label(format!("Name: {}", chicken.name));
                ui.label(format!("Quirks go here, blah blah blah"));
            }) {
                let response = inner_response.response;
                if response.secondary_clicked() {
                    *open = false;
                }
            }
            *just_opened = false;
        }
    }
}
