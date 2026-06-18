use nice_plug_egui::EguiState;
use std::sync::Arc;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const PLUGIN_WIDTH: f32 = 600.0;
const PLUGIN_HEIGHT: f32 = 550.0;

pub(crate) fn default_state() -> Arc<EguiState> {
    EguiState::from_size(PLUGIN_WIDTH as u32, PLUGIN_HEIGHT as u32)
}