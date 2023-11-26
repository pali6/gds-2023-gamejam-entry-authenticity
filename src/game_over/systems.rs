use bevy_egui::{egui, EguiContexts};
use bevy::{prelude::*, window::PrimaryWindow, app::AppExit};
use crate::ambience::AmbiencePlayer;

use crate::states::AppState;


pub fn build_game_over() {
    // TODO
}

pub fn destroy_game_over() {
    // TODO
}

static BUTTON_BACKGROUND: egui::Color32 = egui::Color32::from_rgb(255, 255, 200);
static BUTTON_TEXT_COLOR: egui::Color32 = egui::Color32::from_rgb(0, 0, 0);

fn menu_button(text: &str, ui: &mut egui::Ui) -> egui::Response {
    let rich_text = egui::RichText::new(text).color(BUTTON_TEXT_COLOR);
    ui.add(egui::Button::new(rich_text).fill(BUTTON_BACKGROUND))
}

pub fn ui_game_over(
    mut contexts: EguiContexts,
    mut app_next_state: ResMut<NextState<AppState>>,
    mut main_window: Query<&mut Window, With<PrimaryWindow>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    let ctx = contexts.ctx_mut();
    let mut style = (*ctx.style()).clone();
    style.text_styles.insert(
        egui::TextStyle::Button,
        egui::FontId::new(30.0, egui::FontFamily::Proportional),
    );
    ctx.set_style(style);
    let frame = egui::Frame::default().fill(egui::Color32::from_rgb(50, 140, 50));
    egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
        let layout = egui::Layout::top_down(egui::Align::Center);

        ui.with_layout(layout, |ui| {
            ui.spacing_mut().item_spacing.y = 20.0;
            // reserve some space so we aren't at the very top
            ui.allocate_space(egui::Vec2::new(
                0.0,
                f32::min(80.0, ui.available_size().y / 2.0),
            ));
            ui.label(
                egui::RichText::new("Game Over")
                    .heading()
                    .strong()
                    .color(egui::Color32::from_rgb(255, 255, 200))
                    .size(80.0),
            );
            ui.label(
                egui::RichText::new(crate::help::HELP_TEXT)
                    .color(egui::Color32::from_rgb(255, 255, 255))
                    .size(14.0),
            );
            if menu_button("Start Game", ui).clicked() {
                app_next_state.set(AppState::InGame);
            }
            //if menu_button("Toggle Ambience", ui).clicked() {
            //    toggle_ambience(ambience_players)
            //}
            if !cfg!(target_arch = "wasm32") {
                if menu_button("Toggle Fullscreen", ui).clicked() {
                    let mut main_window = main_window.single_mut();
                    main_window.mode = match main_window.mode {
                        bevy::window::WindowMode::Windowed => bevy::window::WindowMode::Fullscreen,
                        bevy::window::WindowMode::Fullscreen => bevy::window::WindowMode::Windowed,
                        _ => bevy::window::WindowMode::Windowed,
                    };
                }
                if menu_button("Quit", ui).clicked() {
                    app_exit_event_writer.send(AppExit);
                }
            }
        });
    });
}
