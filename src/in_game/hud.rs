use bevy::prelude::*;
use bevy_egui::{egui::{self, RichText}, EguiContexts};

use crate::states::AppState;

#[derive(Resource, Default, Debug)]
struct HudResources {
    fox_head: Handle<Image>,
    chicken_head: Handle<Image>,
    timer_icon: Handle<Image>,
    rendered_fox_head: Option<egui::TextureId>,
    rendered_chicken_head: Option<egui::TextureId>,
    rendered_timer_icon: Option<egui::TextureId>,
}

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<HudResources>()
            .add_systems(Startup, load_hud_resources)
            .add_systems(Update, (ui_hud,)
                .run_if(in_state(AppState::InGame))
            );
    }
}

fn load_hud_resources(
    mut hud_resources: ResMut<HudResources>,
    asset_server: Res<AssetServer>,
) {
    hud_resources.fox_head = asset_server.load("sprites/fox-head-centered.png");
    hud_resources.chicken_head = asset_server.load("sprites/chicken-head-centered.png");
    hud_resources.timer_icon = asset_server.load("sprites/fox-timer.png");
}

fn ui_hud(
    mut contexts: EguiContexts,
    world_parms: Res<crate::world::WorldParams>,
    mut hud_resources: ResMut<HudResources>,
    chicken_hunt: Res<crate::in_game::chicken::chicken_hunt::ChickenHunt>,
) {
    if hud_resources.rendered_fox_head.is_none() {
        hud_resources.rendered_fox_head = Some(contexts.add_image(hud_resources.fox_head.clone_weak()));
        hud_resources.rendered_chicken_head = Some(contexts.add_image(hud_resources.chicken_head.clone_weak()));
        hud_resources.rendered_timer_icon = Some(contexts.add_image(hud_resources.timer_icon.clone_weak()));
    }

    let ctx = contexts.ctx_mut();
    let frame = egui::Frame::default()
        .fill(egui::Color32::from_rgba_unmultiplied(0, 0, 0, 160))
        .rounding(5.0)
        .inner_margin(10.0);
    let window = egui::Window::new("hud")
        .resizable(false)
        .collapsible(false)
        .title_bar(false)
        .frame(frame);
    let text_color = egui::Color32::from_rgb(255, 255, 255);
    window.show(ctx, |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
                    hud_resources.rendered_fox_head.unwrap(),
                    [32.0, 32.0],
                )));
                ui.label(RichText::new(format!("{}/{}", world_parms.foxes_alive, world_parms.fox_count))
                    .strong()
                    .color(text_color)
                    .size(32.0)
                );
            });
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
                    hud_resources.rendered_chicken_head.unwrap(),
                    [32.0, 32.0],
                )));
                ui.label(RichText::new(format!("{}/{}", world_parms.chicken_alive, world_parms.chicken_required))
                    .strong()
                    .color(text_color)
                    .size(32.0)
                );
            });
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
                    hud_resources.rendered_timer_icon.unwrap(),
                    [32.0, 32.0],
                )));
                let timer_time = (chicken_hunt.hunt_timer.duration() - chicken_hunt.hunt_timer.elapsed()).as_secs();
                ui.label(RichText::new(format!("{}s", timer_time))
                    .strong()
                    .color(text_color)
                    .size(32.0)
                );
            });
        });
    });
}