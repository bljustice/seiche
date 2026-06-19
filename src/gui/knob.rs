use std::f32::consts::TAU;

use egui::{
    self, Color32, Pos2, Response, Sense, Shape, Stroke, Ui, Vec2, Widget, emath,
};
use nice_plug::prelude::{Param, ParamSetter};

const ARC_START: f32 = 2.356_194_5; // 135°
const ARC_SWEEP: f32 = 1.5 * TAU; // 270°
const DRAG_SENSITIVITY: f32 = 0.004;

/// Minimal arc knob for nice-plug parameters. The track is a light arc; the value is
/// drawn as a black arc from the start angle up to the current value.
#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct ParamKnob<'a, P: Param> {
    param: &'a P,
    setter: &'a ParamSetter<'a>,
    size: f32,
    stroke_width: f32,
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

    pub fn without_label(mut self) -> Self {
        self.draw_label = false;
        self
    }

    pub fn without_value(mut self) -> Self {
        self.draw_value = false;
        self
    }

    fn normalized_value(&self) -> f32 {
        self.param.modulated_normalized_value()
    }

    fn set_normalized_value(&self, normalized: f32) {
        let value = self.param.preview_plain(normalized);
        if value != self.param.modulated_plain_value() {
            self.setter.set_parameter(self.param, value);
        }
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
        }

        if response.dragged() {
            let delta = -response.drag_delta().y * DRAG_SENSITIVITY;
            let normalized = emath::remap_clamp(
                self.normalized_value() + delta,
                0.0..=1.0,
                0.0..=1.0,
            );
            self.set_normalized_value(normalized);
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
                    ui.label(self.param.name());
                });
            }

            let mut response =
                ui.allocate_response(Vec2::splat(self.size), Sense::click_and_drag());

            self.handle_input(ui, &mut response);
            self.knob_ui(ui, &response);

            if self.draw_value {
                ui.vertical_centered(|ui| {
                    ui.label(self.param.to_string());
                });
            }

            response
        })
        .inner
    }
}
