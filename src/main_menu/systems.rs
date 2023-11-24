
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy::app::AppExit;

use crate::states::AppState;

pub fn build_main_menu() {
    // TODO
}

pub fn destroy_main_menu() {
    // TODO
}

pub fn ui_main_menu(
    mut contexts: EguiContexts,
    mut app_next_state: ResMut<NextState<AppState>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    let frame = egui::Frame::default().fill(egui::Color32::from_rgb(0, 0, 0));
    egui::CentralPanel::default().frame(frame).show(contexts.ctx_mut(), |ui| {
        let layout = egui::Layout::top_down(egui::Align::Center);
        ui.with_layout(layout, |ui| {
            // reserve some space so we aren't at the very top
            ui.allocate_space(egui::Vec2::new(0.0, f32::min(200.0, ui.available_size().y / 2.0)));
            ui.heading("Game title!");
            if ui.button("Start Game").clicked() {
                app_next_state.set(AppState::InGame);
            }
            if ui.button("Quit").clicked() {
                app_exit_event_writer.send(AppExit);
            }
        });
    });
}

pub fn run_if_in_main_menu(
    app_current_state: Res<State<AppState>>,
) -> bool {
    app_current_state.get() == &AppState::MainMenu
}