use bevy::prelude::*;
use bevy_egui::{egui::{self, RichText}, EguiContexts};

use crate::states::AppState;

#[derive(Resource, Default, Debug)]
struct HudResources {
    fox_head: Handle<Image>,
    chicken_head: Handle<Image>,
    rendered_fox_head: Option<egui::TextureId>,
    rendered_chicken_head: Option<egui::TextureId>,
}

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<HudResources>()
            .add_systems(Startup, load_hud_resources)
            .add_systems(PostUpdate, (ui_hud,)
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
}

fn ui_hud(
    mut contexts: EguiContexts,
    world_parms: Res<crate::world::WorldParams>,
    mut hud_resources: ResMut<HudResources>,
) {
    if hud_resources.rendered_fox_head.is_none() {
        hud_resources.rendered_fox_head = Some(contexts.add_image(hud_resources.fox_head.clone_weak()));
        hud_resources.rendered_chicken_head = Some(contexts.add_image(hud_resources.chicken_head.clone_weak()));
    }

    let ctx = contexts.ctx_mut();
    let frame = egui::Frame::default().fill(egui::Color32::TRANSPARENT);
    egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
                    hud_resources.rendered_fox_head.unwrap(),
                    [32.0, 32.0],
                )));
                ui.label(RichText::new(format!("{}/{}", world_parms.foxes_alive, world_parms.fox_count))
                    .strong()
                    .color(egui::Color32::from_rgb(0, 0, 0))
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
                    .color(egui::Color32::from_rgb(0, 0, 0))
                    .size(32.0)
                );
            });
        });
    });
}