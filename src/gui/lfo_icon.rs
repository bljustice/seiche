use egui::{
    Align2, Color32, Popup, Rect, Sense, Shape, Stroke, StrokeKind, Ui, Vec2,
};
use egui::style::WidgetVisuals;
use nice_plug::prelude::{EnumParam, ParamSetter};

use crate::params::LfoType;

const LFO_TYPES: [LfoType; 4] = [
    LfoType::Sine,
    LfoType::Square,
    LfoType::Random,
    LfoType::Ramp,
];

const ICON_SIZE: Vec2 = Vec2::new(40.0, 18.0);
const COMBO_WIDTH: f32 = 56.0;
/// Combo button fill. Use `Color32::TRANSPARENT` to show the panel through, or `Color32::WHITE`.
const COMBO_BG: Color32 = Color32::TRANSPARENT;

pub fn paint_lfo_icon(painter: &egui::Painter, rect: Rect, lfo: LfoType) {
    let stroke = Stroke::new(1.5, egui::Color32::BLACK);
    let n = 32;
    let points: Vec<egui::Pos2> = (0..n)
        .map(|i| {
            let t = i as f32 / (n - 1) as f32;
            let y = match lfo {
                LfoType::Sine => (t * std::f32::consts::TAU).sin(),
                LfoType::Square => if t < 0.5 { 1.0 } else { -1.0 },
                LfoType::Ramp => 1.0 - 2.0 * t,
                LfoType::Random => (t * 12.7).sin() * 0.6 + (t * 4.3).cos() * 0.4,
            };
            let x = rect.left() + t * rect.width();
            let py = rect.center().y - y * rect.height() * 0.4;
            egui::pos2(x, py)
        })
        .collect();
    painter.add(Shape::line(points, stroke));
}

pub fn draw_lfo_icon(ui: &mut Ui, lfo: LfoType, size: Vec2) {
    let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
    paint_lfo_icon(ui.painter(), rect, lfo);
}

fn paint_combo_arrow(painter: &egui::Painter, rect: Rect, visuals: &WidgetVisuals) {
    let rect = Rect::from_center_size(
        rect.center(),
        Vec2::new(rect.width() * 0.7, rect.height() * 0.45),
    );

    painter.add(Shape::convex_polygon(
        vec![rect.left_top(), rect.right_top(), rect.center_bottom()],
        visuals.fg_stroke.color,
        Stroke::NONE,
    ));
}

pub fn lfo_type_combo(ui: &mut Ui, lfo: &EnumParam<LfoType>, setter: &ParamSetter<'_>) {
    let current = lfo.value();
    let button_id = ui.make_persistent_id("lfo_type");
    let popup_id = button_id.with("popup");
    let is_open = Popup::is_id_open(ui.ctx(), popup_id);

    let margin = ui.spacing().button_padding;
    let icon_spacing = ui.spacing().icon_spacing;
    let arrow_size = Vec2::splat(ui.spacing().icon_width);
    let inner_height = ICON_SIZE.y.max(arrow_size.y);
    let size = Vec2::new(COMBO_WIDTH, inner_height + margin.y * 2.0);

    let response = ui.allocate_response(size, Sense::click());

    let visuals = if is_open {
        &ui.visuals().widgets.open
    } else {
        ui.style().interact(&response)
    };

    if ui.is_rect_visible(response.rect) {
        let bg = if response.hovered() || is_open {
            visuals.weak_bg_fill
        } else {
            COMBO_BG
        };

        ui.painter().rect(
            response.rect.expand(visuals.expansion),
            visuals.corner_radius,
            bg,
            visuals.bg_stroke,
            StrokeKind::Inside,
        );

        let inner = response.rect.shrink2(margin);
        let arrow_rect = Align2::RIGHT_CENTER.align_size_within_rect(arrow_size, inner);
        paint_combo_arrow(ui.painter(), arrow_rect, visuals);

        let icon_area = Rect::from_min_size(
            inner.min,
            Vec2::new(
                (inner.width() - icon_spacing - arrow_size.x).max(0.0),
                inner.height(),
            ),
        );
        let icon_rect = Align2::LEFT_CENTER.align_size_within_rect(ICON_SIZE, icon_area);
        paint_lfo_icon(ui.painter(), icon_rect, current);
    }

    Popup::menu(&response)
        .id(popup_id)
        .width(response.rect.width())
        .show(|ui| {
            for lfo_type in LFO_TYPES {
                ui.horizontal(|ui| {
                    draw_lfo_icon(ui, lfo_type, ICON_SIZE);

                    if ui
                        .selectable_label(current == lfo_type, lfo_type.label())
                        .clicked()
                    {
                        setter.begin_set_parameter(lfo);
                        setter.set_parameter(lfo, lfo_type);
                        setter.end_set_parameter(lfo);
                    }
                });
            }
        });
}
