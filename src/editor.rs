use egui::{Color32, FontData, FontDefinitions, FontFamily, Margin};
use nice_plug::prelude::Editor;
use nice_plug_egui::{create_egui_editor, EguiSettings, EguiState};
use std::sync::Arc;

use crate::gui::knob::ParamKnob;
use crate::params::{TremoloMode, TremoloParams};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const PLUGIN_WIDTH: f32 = 600.0;
const PLUGIN_HEIGHT: f32 = 550.0;

pub(crate) fn default_state() -> Arc<EguiState> {
    EguiState::from_size(PLUGIN_WIDTH as u32, PLUGIN_HEIGHT as u32)
}

pub(crate) fn create(params: Arc<TremoloParams>, editor_state: Arc<EguiState>) -> Option<Box<dyn Editor>> {
    create_egui_editor(
        editor_state.clone(),
        (),
        EguiSettings::default(),
        |ctx, _queue, _state| {

            let mut fonts = FontDefinitions::default();

            fonts.font_data.insert(
                "seiche".to_owned(),
                Arc::new(FontData::from_static(include_bytes!(
                    "./assets/fonts/NotoSansDisplay-VariableFont_wdth,wght.ttf"
                ))),
            );

            fonts
                .families
                .entry(FontFamily::Proportional)
                .or_default()
                .insert(0, "seiche".to_owned());

            fonts
                .families
                .insert(FontFamily::Name("heading".into()), vec!["seiche".to_owned()]);

            ctx.set_fonts(fonts);

            ctx.global_style_mut(|style| {
                style.visuals.window_fill = Color32::from_rgb(0xF9, 0xF6, 0xEE);
                style.visuals.panel_fill = Color32::from_rgb(0xF9, 0xF6, 0xEE);
                style.visuals.override_text_color = Some(Color32::BLACK);
            });
        },
        move |ui, setter, _queue, _state| {
            egui::CentralPanel::default()
                .frame(
                    egui::Frame::new()
                        .inner_margin(Margin::same(12))
                        .fill(Color32::from_rgb(0xF9, 0xF6, 0xEE)),
                )
                .show_inside(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading(
                            egui::RichText::new("Seiche")
                                .family(FontFamily::Name("heading".into()))
                                .size(24.0),
                        );
                        ui.label(format!("Version: {}", VERSION));
                    });

                    ui.add_space(16.0);

                    ui.horizontal(|ui| {
                        let mode = params.mode.value();
                        if ui.radio(mode == TremoloMode::Classic, "Classic").clicked() {
                            setter.begin_set_parameter(&params.mode);
                            setter.set_parameter(&params.mode, TremoloMode::Classic);
                            setter.end_set_parameter(&params.mode);
                        }
                        if ui.radio(mode == TremoloMode::Harmonic, "Harmonic").clicked() {
                            setter.begin_set_parameter(&params.mode);
                            setter.set_parameter(&params.mode, TremoloMode::Harmonic);
                            setter.end_set_parameter(&params.mode);
                        }
                    });

                    ui.vertical(|ui| {
                        ui.add(ParamKnob::for_param(&params.depth, setter));
                        ui.add(ParamKnob::for_param(&params.rate, setter));
                    });
                });
        },
    )
}