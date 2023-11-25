use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContexts};

use crate::in_game::states::InGameState;
use crate::states::AppState;

pub fn build_pause_menu() {
    // TODO
}

pub fn destroy_pause_menu() {
    // TODO
}

static BUTTON_BACKGROUND: egui::Color32 = egui::Color32::from_rgb(255, 255, 200);
static BUTTON_TEXT_COLOR: egui::Color32 = egui::Color32::from_rgb(0, 0, 0);

fn menu_button(text: &str, ui: &mut egui::Ui) -> egui::Response {
    let rich_text = egui::RichText::new(text).color(BUTTON_TEXT_COLOR);
    ui.add(egui::Button::new(rich_text).fill(BUTTON_BACKGROUND))
}

pub fn ui_pause_menu(
    mut contexts: EguiContexts,
    mut ingame_next_state: ResMut<NextState<InGameState>>,
    mut app_next_state: ResMut<NextState<AppState>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut main_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    let ctx = contexts.ctx_mut();
    let mut style = (*ctx.style()).clone();
    style.text_styles.insert(
        egui::TextStyle::Button,
        egui::FontId::new(30.0, egui::FontFamily::Proportional),
    );
    ctx.set_style(style);
    let frame = egui::Frame::default().fill(egui::Color32::from_rgba_unmultiplied(0, 0, 0, 200));
    egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
        let layout = egui::Layout::top_down(egui::Align::Center);
        ui.with_layout(layout, |ui| {
            ui.spacing_mut().item_spacing.y = 20.0;
            // reserve some space so we aren't at the very top
            ui.allocate_space(egui::Vec2::new(
                0.0,
                f32::min(100.0, ui.available_size().y / 2.0),
            ));
            ui.label(
                egui::RichText::new("PAUSED")
                    .heading()
                    .strong()
                    .color(egui::Color32::from_rgb(255, 255, 255))
                    .size(80.0),
            );
            ui.label(
                egui::RichText::new(crate::help::HELP_TEXT)
                    .color(egui::Color32::from_rgb(255, 255, 255))
                    .size(14.0),
            );
            if menu_button("Unpause", ui).clicked() {
                ingame_next_state.set(InGameState::Running);
            }
            if menu_button("Toggle Fullscreen", ui).clicked() {
                let mut main_window = main_window.single_mut();
                main_window.mode = match main_window.mode {
                    bevy::window::WindowMode::Windowed => bevy::window::WindowMode::Fullscreen,
                    bevy::window::WindowMode::Fullscreen => bevy::window::WindowMode::Windowed,
                    _ => bevy::window::WindowMode::Windowed,
                };
            }
            if menu_button("Main Menu", ui).clicked() {
                app_next_state.set(AppState::MainMenu);
            }
            if menu_button("Quit", ui).clicked() {
                app_exit_event_writer.send(AppExit);
            }
        });
    });
}

pub fn run_if_in_pause_menu(
    app_current_state: Res<State<AppState>>,
    in_game_state: Res<State<InGameState>>,
) -> bool {
    app_current_state.get() == &AppState::InGame && in_game_state.get() == &InGameState::Paused
}
