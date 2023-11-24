use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::in_game::states::InGameState;
use crate::states::AppState;

pub fn build_pause_menu() {
    // TODO
}

pub fn destroy_pause_menu() {
    // TODO
}

pub fn ui_pause_menu(
    mut contexts: EguiContexts,
    mut ingame_next_state: ResMut<NextState<InGameState>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    let ctx = contexts.ctx_mut();
    let mut style = (*ctx.style()).clone();
    style.text_styles.insert(
        egui::TextStyle::Button,
        egui::FontId::new(30.0, egui::FontFamily::Proportional),
    );
    ctx.set_style(style);
    egui::CentralPanel::default().show(ctx, |ui| {
        let layout = egui::Layout::top_down(egui::Align::Center);
        ui.with_layout(layout, |ui| {
            ui.spacing_mut().item_spacing.y = 20.0;
            // reserve some space so we aren't at the very top
            ui.allocate_space(egui::Vec2::new(
                0.0,
                f32::min(150.0, ui.available_size().y / 2.0),
            ));
            ui.label(
                egui::RichText::new("PAUSED")
                    .heading()
                    .strong()
                    .color(egui::Color32::from_rgb(255, 255, 255))
                    .size(80.0),
            );
            if ui.button("Unpause").clicked() {
                ingame_next_state.set(InGameState::Running);
            }
            if ui.button("Quit").clicked() {
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
