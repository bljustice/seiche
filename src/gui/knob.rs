use std::f32::consts::TAU;

use egui::{
    self, Color32, Pos2, Response, Sense, Shape, Stroke, Ui, Vec2, Widget, emath,
};
use nice_plug::prelude::{Param, ParamSetter};

const ARC_START: f32 = 2.356_194_5; // 135°
const ARC_SWEEP: f32 = 0.75 * TAU; // 270° (3/4 turn)
const CONTINUOUS_DRAG_SENSITIVITY: f32 = 0.004;
const DISCRETE_DRAG_THRESHOLD: f32 = 6.0;

/// Minimal arc knob for nice-plug parameters. The track is a light arc; the value is
/// drawn as a black arc from the start angle up to the current value
pub struct ParamKnob<'a, P: Param> {
    param: &'a P,
    setter: &'a ParamSetter<'a>,
    size: f32,
    stroke_width: f32,
    label_value: Option<String>,
    param_value: Option<String>,
    draw_label: bool,
    draw_value: bool,
}

impl<'a, P: Param> ParamKnob<'a, P> {
    pub fn for_param(param: &'a P, setter: &'a ParamSetter<'a>) -> Self {
        Self {
            param,
            setter,
            size: 56.0,
            stroke_width: 2.0,
            label_value: None,
            param_value: None,
            draw_label: true,
            draw_value: true,
        }
    }

    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn with_stroke_width(mut self, stroke_width: f32) -> Self {
        self.stroke_width = stroke_width;
        self
    }

    pub fn with_label(mut self, label: String) -> Self {
        self.label_value = Some(label);
        self
    }

    pub fn without_label(mut self) -> Self {
        self.draw_label = false;
        self
    }

    pub fn with_value(mut self, value: String) -> Self {
        self.param_value = Some(value);
        self
    }

    pub fn without_value(mut self) -> Self {
        self.draw_value = false;
        self
    }

    fn normalized_value(&self) -> f32 {
        self.param.modulated_normalized_value()
    }

    fn discrete_drag_id(ui: &Ui, param: &P) -> egui::Id {
        ui.auto_id_with(("knob_discrete_drag", param.name()))
    }

    fn set_plain_value(&self, value: P::Plain) {
        if value != self.param.modulated_plain_value() {
            self.setter.set_parameter(self.param, value);
        }
    }

    fn set_normalized_value(&self, normalized: f32) {
        self.set_plain_value(self.param.preview_plain(normalized));
    }

    fn reset_param(&self) {
        self.setter
            .set_parameter(self.param, self.param.default_plain_value());
    }

    fn value_angle(normalized: f32) -> f32 {
        ARC_START + normalized * ARC_SWEEP
    }

    fn draw_arc(
        painter: &egui::Painter,
        center: Pos2,
        radius: f32,
        stroke: Stroke,
        start: f32,
        end: f32,
    ) {
        let steps = ((end - start).abs() / 0.04).ceil() as usize;
        let steps = steps.clamp(2, 128);
        let points: Vec<Pos2> = (0..=steps)
            .map(|i| {
                let t = i as f32 / steps as f32;
                let angle = start + t * (end - start);
                center + Vec2::angled(angle) * radius
            })
            .collect();
        painter.add(Shape::line(points, stroke));
    }

    fn knob_ui(&self, ui: &Ui, response: &Response) {
        if !ui.is_rect_visible(response.rect) {
            return;
        }

        let center = response.rect.center();
        let radius = response.rect.width() * 0.38;
        let track_stroke = Stroke::new(
            self.stroke_width,
            Color32::from_gray(190),
        );
        let value_stroke = Stroke::new(self.stroke_width, Color32::BLACK);

        Self::draw_arc(
            ui.painter(),
            center,
            radius,
            track_stroke,
            ARC_START,
            ARC_START + ARC_SWEEP,
        );

        let normalized = self.normalized_value();
        if normalized > 0.0 {
            Self::draw_arc(
                ui.painter(),
                center,
                radius,
                value_stroke,
                ARC_START,
                Self::value_angle(normalized),
            );
        }
        
        if response.hovered() || response.dragged() {
            ui.painter().circle_filled(
                center + Vec2::angled(Self::value_angle(normalized)) * radius,
                self.stroke_width * 0.9,
                Color32::BLACK,
            );
        }
    }

    fn handle_input(&self, ui: &Ui, response: &mut Response) {
        if response.double_clicked() || (response.clicked() && ui.input(|i| i.modifiers.command)) {
            self.setter.begin_set_parameter(self.param);
            self.reset_param();
            self.setter.end_set_parameter(self.param);
            response.mark_changed();
            return;
        }

        if response.drag_started() {
            self.setter.begin_set_parameter(self.param);
            ui.memory_mut(|mem| {
                mem.data
                    .insert_temp(Self::discrete_drag_id(ui, self.param), 0.0f32);
            });
        }

        if response.dragged() {
            let finer = ui.input(|i| i.modifiers.shift);
            let delta = -response.drag_delta().y;

            if self.param.step_count().is_some() {
                let drag_id = Self::discrete_drag_id(ui, self.param);
                let mut accumulated: f32 = ui.memory(|mem| {
                    mem.data.get_temp(drag_id).unwrap_or(0.0)
                });
                accumulated += delta;

                while accumulated >= DISCRETE_DRAG_THRESHOLD {
                    let normalized = self.normalized_value();
                    let next = self.param.next_normalized_step(normalized, finer);
                    if (next - normalized).abs() < f32::EPSILON {
                        break;
                    }
                    self.set_normalized_value(next);
                    accumulated -= DISCRETE_DRAG_THRESHOLD;
                }

                while accumulated <= -DISCRETE_DRAG_THRESHOLD {
                    let normalized = self.normalized_value();
                    let previous = self.param.previous_normalized_step(normalized, finer);
                    if (previous - normalized).abs() < f32::EPSILON {
                        break;
                    }
                    self.set_normalized_value(previous);
                    accumulated += DISCRETE_DRAG_THRESHOLD;
                }

                ui.memory_mut(|mem| mem.data.insert_temp(drag_id, accumulated));
            } else {
                let normalized = emath::remap_clamp(
                    self.normalized_value() + delta * CONTINUOUS_DRAG_SENSITIVITY,
                    0.0..=1.0,
                    0.0..=1.0,
                );
                self.set_normalized_value(normalized);
            }

            response.mark_changed();
        }

        if response.drag_stopped() {
            self.setter.end_set_parameter(self.param);
        }
    }
}

impl<P: Param> Widget for ParamKnob<'_, P> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.set_width(self.size);

            if self.draw_label {
                ui.vertical_centered(|ui| {
                    match &self.label_value {
                        Some(label) => ui.label(label),
                        None => ui.label(self.param.name()),
                    }
                });
            }

            let mut response =
                ui.allocate_response(Vec2::splat(self.size), Sense::click_and_drag());

            self.handle_input(ui, &mut response);
            self.knob_ui(ui, &response);

            if self.draw_value {
                ui.vertical_centered(|ui| {
                    match &self.param_value {
                        Some(value) => ui.label(value),
                        None => ui.label(self.param.to_string()),
                    }
                });
            }

            response
        })
        .inner
    }
}
