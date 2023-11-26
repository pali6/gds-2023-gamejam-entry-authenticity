use std::collections::HashMap;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContexts};

use crate::in_game::states::InGameState;
use crate::states::AppState;

use super::components::Chicken;
use super::click::ChickenClickEvent;

pub struct InfoMenuPlugin;

impl Plugin for InfoMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ChickenMenus>()
            .add_systems(OnEnter(AppState::InGame), reset_chicken_menus)
            .add_systems(Update, (open_chicken_menu, display_menus)
                .run_if(in_state(AppState::InGame).and_then(in_state(InGameState::Running)))
            );
    }
}

#[derive(Debug)]
struct WindowState {
    open: bool,
    just_opened: bool,
}

#[derive(Resource, Default, Debug)]
pub struct ChickenMenus {
    chickens: HashMap<Entity, WindowState>,
    pub mouse_over_menu: bool,
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

#[derive(Component, Default)]
pub struct HasEguiIcon(Option<egui::TextureId>);

fn display_menus(
    mut menus: ResMut<ChickenMenus>,
    query: Query<(&Chicken, &Children)>,
    mut with_egui_icons: Query<(&Handle<Image>, &mut HasEguiIcon)>,
    mut egui_contexts: EguiContexts,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    for (entity, _ ) in menus.chickens.iter_mut() {
        if let Ok((_, children)) = query.get(*entity) {
            for child in children.iter() {
                if let Ok((image_handle, mut has_egui_icon)) = with_egui_icons.get_mut(*child) {
                    if has_egui_icon.0.is_none() {
                        has_egui_icon.0 = Some(egui_contexts.add_image(image_handle.clone_weak()));
                    }
                    break;
                }
            }
        }
    }

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

    let over_ui_last = menus.mouse_over_menu;

    let mut hovering_over = false;

    for (entity, WindowState {open, just_opened} ) in menus.chickens.iter_mut() {
        if let Ok((chicken, children)) = query.get(*entity) {
            let mut egui_icon = None;
            for child in children.iter() {
                if let Ok((_, has_egui_icon)) = with_egui_icons.get_mut(*child) {
                    egui_icon = has_egui_icon.0;
                    break;
                }
            }
            let frame = egui::Frame::default()
                .rounding(5.0)
                .outer_margin(2.0)
                .fill(
                    if over_ui_last {
                        egui::Color32::from_rgba_unmultiplied(200, 200, 200, 180)
                    } else {
                        egui::Color32::from_rgba_unmultiplied(200, 200, 200, 130)
                    }
                );
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
                if let Some(egui_icon) = egui_icon {
                    ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
                        egui_icon,
                        [32.0, 32.0],
                    )));
                }
                for (_quirk, quirk_desc) in chicken.quirks.iter() {
                    ui.label(quirk_desc);
                }
            }) {
                let response = inner_response.response;
                if response.secondary_clicked() {
                    *open = false;
                }
                hovering_over = hovering_over || response.hovered() || response.dragged();
                if let Some(pointer_pos) = mouse_position {
                    hovering_over = hovering_over || response.rect.contains(egui::Pos2::new(pointer_pos.x, pointer_pos.y));
                }
            }
            *just_opened = false;
        }
    }

    menus.mouse_over_menu = hovering_over;
}
