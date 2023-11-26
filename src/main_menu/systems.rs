use bevy::{app::AppExit, window::PrimaryWindow};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::ambience::{toggle_ambience, AmbiencePlayer};
use crate::states::AppState;
use crate::world::WorldParams;

pub fn build_main_menu() {
    // TODO
}

pub fn destroy_main_menu() {
    // TODO
}

static BUTTON_BACKGROUND: egui::Color32 = egui::Color32::from_rgb(255, 255, 200);
static BUTTON_TEXT_COLOR: egui::Color32 = egui::Color32::from_rgb(0, 0, 0);

fn menu_button(text: &str, ui: &mut egui::Ui) -> egui::Response {
    let rich_text = egui::RichText::new(text).color(BUTTON_TEXT_COLOR);
    ui.add(egui::Button::new(rich_text).fill(BUTTON_BACKGROUND))
}

pub fn ui_main_menu(
    mut contexts: EguiContexts,
    mut app_next_state: ResMut<NextState<AppState>>,
    mut main_window: Query<&mut Window, With<PrimaryWindow>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
    ambience_players: Query<&mut AudioSink, With<AmbiencePlayer>>,
    mut world_params: ResMut<WorldParams>,
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
                f32::min(40.0, ui.available_size().y / 2.0),
            ));
            ui.label(
                egui::RichText::new("aut-HEN-tic")
                    .heading()
                    .strong()
                    .color(egui::Color32::from_rgb(255, 255, 200))
                    .size(80.0),
            );
            ui.label(
                egui::RichText::new(crate::help::HELP_TEXT)
                    .color(egui::Color32::from_rgb(255, 255, 255))
                    .size(14.0)
                    .strong(),
            );
            ui.allocate_ui_with_layout(
                egui::Vec2::new(500.0, 0.0),
                egui::Layout::left_to_right(egui::Align::Center).with_cross_align(egui::Align::Center), 
                |ui| {
                    if menu_button("Start Easy", ui).clicked() {
                        world_params.apply_difficulty_preset(&crate::world::EASY);
                        app_next_state.set(AppState::InGame);
                    }
                    if menu_button("Start Medium", ui).clicked() {
                        world_params.apply_difficulty_preset(&crate::world::MEDIUM);
                        app_next_state.set(AppState::InGame);
                    }
                    if menu_button("Start Hard", ui).clicked() {
                        world_params.apply_difficulty_preset(&crate::world::HARD);
                        app_next_state.set(AppState::InGame);
                    }
            });
            if menu_button("Toggle Ambience", ui).clicked() {
                toggle_ambience(ambience_players)
            }
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

pub fn run_if_in_main_menu(app_current_state: Res<State<AppState>>) -> bool {
    app_current_state.get() == &AppState::MainMenu
}
