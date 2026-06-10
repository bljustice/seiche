use nice_plug::prelude::{
    formatters, Enum, EnumParam, FloatParam, FloatRange, Params, SmoothingStyle,
};

use crate::lfo::Waveform;

#[derive(Enum, PartialEq, Debug)]
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

#[derive(Params)]
pub struct TremoloParams {
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
    #[id = "crossover_frequency"]
    pub crossover_frequency: FloatParam,
}

impl TremoloParams {
    pub fn new() -> Self {
        Self {
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
            crossover_frequency: FloatParam::new(
                "Crossover Frequency",
                4000.0,
                FloatRange::Linear { min: 10.0, max: 20000.0 }
            )
            .with_value_to_string(formatters::v2s_f32_hz_then_khz(2))
            .with_string_to_value(formatters::s2v_f32_hz_then_khz()),
        }
    }
}