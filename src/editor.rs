use egui::{Color32, Margin};
use nice_plug::prelude::Editor;
use nice_plug_egui::{create_egui_editor, EguiSettings, EguiState};
use std::sync::Arc;

use crate::params::TremoloParams;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const PLUGIN_WIDTH: f32 = 600.0;
const PLUGIN_HEIGHT: f32 = 550.0;

pub(crate) fn default_state() -> Arc<EguiState> {
    EguiState::from_size(PLUGIN_WIDTH as u32, PLUGIN_HEIGHT as u32)
}

pub(crate) fn create(_params: Arc<TremoloParams>, editor_state: Arc<EguiState>) -> Option<Box<dyn Editor>> {
    create_egui_editor(
        editor_state.clone(),
        (),
        EguiSettings::default(),
        |ctx, _queue, _state| {
            ctx.global_style_mut(|style| {
                style.visuals.window_fill = Color32::from_rgb(0xF9, 0xF6, 0xEE);
                style.visuals.panel_fill = Color32::from_rgb(0xF9, 0xF6, 0xEE);
                style.visuals.override_text_color = Some(Color32::BLACK);
            });
        },
        move |ui, _setter, _queue, _state| {
            egui::CentralPanel::default()
                .frame(
                    egui::Frame::new()
                        .inner_margin(Margin::same(12))
                        .fill(Color32::from_rgb(0xF9, 0xF6, 0xEE)),
                )
                .show_inside(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Seiche");
                        ui.label(format!("Version: {}", VERSION));
                    });
                });
        },
    )
}