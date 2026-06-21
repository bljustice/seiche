use std::sync::Arc;

use nice_plug::prelude::{Editor, Enum, Param, ParamSetter};
use nice_plug_slint::SlintEditor;

use crate::gui;
use crate::params::{LfoType, RateMode, SyncDivision, TremoloMode, TremoloParams};

fn enum_normalized(index: usize, variant_count: usize) -> f32 {
    if variant_count <= 1 {
        0.0
    } else {
        index as f32 / (variant_count - 1) as f32
    }
}

pub(crate) fn create(params: Arc<TremoloParams>) -> Option<Box<dyn Editor>> {
    Some(Box::new(
        SlintEditor::new(params.editor_state.clone(), || gui::AppWindow::new())
            .with_setup({
                let params = params.clone();
                move |handler, _window| {
                    let component = handler.component();
                    let context = handler.context().clone();

                    let params_depth = params.clone();
                    let context_depth = context.clone();
                    component.on_depth_changed(move |value| {
                        let setter = ParamSetter::new(&*context_depth);
                        setter.begin_set_parameter(&params_depth.depth);
                        setter.set_parameter_normalized(&params_depth.depth, value);
                        setter.end_set_parameter(&params_depth.depth);
                    });
                    let params_depth = params.clone();
                    let context_depth = context.clone();
                    component.on_depth_begin_drag(move || {
                        ParamSetter::new(&*context_depth).begin_set_parameter(&params_depth.depth);
                    });
                    let params_depth = params.clone();
                    let context_depth = context.clone();
                    component.on_depth_end_drag(move || {
                        ParamSetter::new(&*context_depth).end_set_parameter(&params_depth.depth);
                    });

                    let params_rate = params.clone();
                    let context_rate = context.clone();
                    component.on_rate_changed(move |value| {
                        let setter = ParamSetter::new(&*context_rate);
                        setter.begin_set_parameter(&params_rate.rate);
                        setter.set_parameter_normalized(&params_rate.rate, value);
                        setter.end_set_parameter(&params_rate.rate);
                    });
                    let params_rate = params.clone();
                    let context_rate = context.clone();
                    component.on_rate_begin_drag(move || {
                        ParamSetter::new(&*context_rate).begin_set_parameter(&params_rate.rate);
                    });
                    let params_rate = params.clone();
                    let context_rate = context.clone();
                    component.on_rate_end_drag(move || {
                        ParamSetter::new(&*context_rate).end_set_parameter(&params_rate.rate);
                    });

                    let params_mode = params.clone();
                    let context_mode = context.clone();
                    component.on_mode_changed(move |index| {
                        let setter = ParamSetter::new(&*context_mode);
                        setter.begin_set_parameter(&params_mode.mode);
                        setter.set_parameter_normalized(
                            &params_mode.mode,
                            enum_normalized(index as usize, TremoloMode::variants().len()),
                        );
                        setter.end_set_parameter(&params_mode.mode);
                    });

                    let params_rate_mode = params.clone();
                    let context_rate_mode = context.clone();
                    component.on_rate_mode_changed(move |index| {
                        let setter = ParamSetter::new(&*context_rate_mode);
                        setter.begin_set_parameter(&params_rate_mode.rate_mode);
                        setter.set_parameter_normalized(
                            &params_rate_mode.rate_mode,
                            enum_normalized(index as usize, RateMode::variants().len()),
                        );
                        setter.end_set_parameter(&params_rate_mode.rate_mode);
                    });

                    let params_sync = params.clone();
                    let context_sync = context.clone();
                    component.on_sync_division_changed(move |index| {
                        let setter = ParamSetter::new(&*context_sync);
                        setter.begin_set_parameter(&params_sync.sync_division);
                        setter.set_parameter_normalized(
                            &params_sync.sync_division,
                            enum_normalized(index as usize, SyncDivision::variants().len()),
                        );
                        setter.end_set_parameter(&params_sync.sync_division);
                    });

                    let params_lfo = params.clone();
                    let context_lfo = context;
                    component.on_lfo_changed(move |index| {
                        let setter = ParamSetter::new(&*context_lfo);
                        setter.begin_set_parameter(&params_lfo.lfo);
                        setter.set_parameter_normalized(
                            &params_lfo.lfo,
                            enum_normalized(index as usize, LfoType::variants().len()),
                        );
                        setter.end_set_parameter(&params_lfo.lfo);
                    });
                }
            })
            .with_event_loop({
                let params = params.clone();
                move |handler, _setter, _window| {
                    let component = handler.component();
                    component.set_depth_value(params.depth.modulated_normalized_value());
                    component.set_depth_label(params.depth.to_string().into());
                    component.set_rate_value(params.rate.modulated_normalized_value());
                    component.set_rate_label(params.rate.to_string().into());
                    component.set_mode_index(params.mode.value().to_index() as i32);
                    component.set_rate_mode_index(params.rate_mode.value().to_index() as i32);
                    component.set_lfo_index(params.lfo.value().to_index() as i32);
                    component.set_sync_division_index(
                        params.sync_division.value().to_index() as i32,
                    );
                    component.set_sync_mode(params.rate_mode.value() == RateMode::Sync);
                }
            }),
    ))
}
