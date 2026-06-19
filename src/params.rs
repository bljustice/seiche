use nice_plug::{prelude::{
    Enum, EnumParam, FloatParam, FloatRange, Params, SmoothingStyle, formatters
}};
use std::sync::Arc;
use nice_plug_egui::EguiState;
use crate::editor;
use crate::lfo::Waveform;

#[derive(Enum, PartialEq, Copy, Clone, Debug)]
pub enum LfoType {
    #[id = "sine"]
    Sine,
    #[id = "square"]
    Square,
    #[id = "random"]
    Random,
    #[id = "ramp"]
    Ramp,
}

#[derive(Enum, PartialEq, Debug)]
pub enum TremoloMode {
    #[id = "classic"]
    Classic,
    #[id = "harmonic"]
    Harmonic,
}

impl LfoType {
    pub fn label(&self) -> &'static str {
        match self {
            LfoType::Sine => "Sine",
            LfoType::Square => "Square",
            LfoType::Random => "Random",
            LfoType::Ramp => "Ramp",
        }
    }
}

impl From<LfoType> for Waveform {
    fn from(t: LfoType) -> Self {
        match t {
            LfoType::Sine => Waveform::Sine,
            LfoType::Square => Waveform::Square,
            LfoType::Random => Waveform::Random,
            LfoType::Ramp => Waveform::Ramp,
        }
    }
}

#[derive(Enum, PartialEq, Debug)]
pub enum RateMode {
    #[id = "free"]
    Free,
    #[id = "sync"]
    Sync,
}

#[derive(Enum, PartialEq, Debug)]
pub enum SyncDivision {
    #[id = "1/1"]
    Whole,
    #[id = "1/2"]
    Half,
    #[id = "1/4"]
    Quarter,
    #[id = "1/8"]
    Eighth,
    #[id = "1/16"]
    Sixteenth,
    #[id = "1/32"]  
    ThirtySecond,
}

impl SyncDivision {
    pub fn beats_per_cycle(self) -> f32 {
        match self {
            Self::Whole => 4.0,
            Self::Half => 2.0,
            Self::Quarter => 1.0,
            Self::Eighth => 0.5,
            Self::Sixteenth => 0.25,
            Self::ThirtySecond => 0.125,
        }
    }
}

#[derive(Params)]
pub struct TremoloParams {
    #[persist = "editor-state"]
    pub editor_state: Arc<EguiState>,
    #[id = "mode"]
    pub mode: EnumParam<TremoloMode>,
    #[id = "depth"] 
    pub depth: FloatParam,
    #[id = "rate"]
    pub rate: FloatParam,
    #[id = "lfo"]
    pub lfo: EnumParam<LfoType>,
    #[id = "flip_probability"]
    pub flip_probability: FloatParam,
    #[id = "bias"]
    pub bias: FloatParam,
    #[id = "jitter"]
    pub jitter: FloatParam,
    #[id = "crossover_frequency"]
    pub crossover_frequency: FloatParam,
    #[id = "rate_mode"]
    pub rate_mode: EnumParam<RateMode>,
    #[id = "sync_division"]
    pub sync_division: EnumParam<SyncDivision>,
}

impl TremoloParams {
    pub fn new() -> Self {
        Self {
            editor_state: editor::default_state(),
            mode: EnumParam::new("Mode", TremoloMode::Classic),
            depth: FloatParam::new(
                "Depth",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 }
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_unit(" %")
            .with_value_to_string(formatters::v2s_f32_percentage(2))
            .with_string_to_value(formatters::s2v_f32_percentage()),
            rate: FloatParam::new(
                "Rate",
                4.0,
                FloatRange::Skewed {
                    min: 0.01,
                    max: 32.0,
                    factor: FloatRange::skew_factor(-2.0),
                }
            )
            .with_value_to_string(formatters::v2s_f32_hz_then_khz(2))
            .with_string_to_value(formatters::s2v_f32_hz_then_khz()),
            lfo: EnumParam::new("LFO", LfoType::Sine),
            flip_probability: FloatParam::new(
                "Flip Probability",
                0.3,
                FloatRange::Linear { min: 0.0, max: 1.0 }
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_unit(" %")
            .with_value_to_string(formatters::v2s_f32_percentage(2))
            .with_string_to_value(formatters::s2v_f32_percentage()),
            bias: FloatParam::new(
                "Bias",
                0.2,
                FloatRange::Linear { min: -1.0, max: 1.0 }
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_value_to_string(formatters::v2s_f32_rounded(2)),
            jitter: FloatParam::new(
                "Jitter",
                0.0,
                FloatRange::Linear { min: 0.0, max: 1.0 }
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_unit(" %")
            .with_value_to_string(formatters::v2s_f32_percentage(2))
            .with_string_to_value(formatters::s2v_f32_percentage()),
            crossover_frequency: FloatParam::new(
                "Crossover Frequency",
                400.0,
                FloatRange::Skewed {
                    min: 5.0,
                    max: 20_000.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(100.0))
            .with_value_to_string(formatters::v2s_f32_hz_then_khz(2))
            .with_string_to_value(formatters::s2v_f32_hz_then_khz()),
            rate_mode: EnumParam::new("Rate Mode", RateMode::Free),
            sync_division: EnumParam::new("Sync Division", SyncDivision::Quarter),
        }
    }
}