use nice_plug::prelude::{
    formatters, util, Enum, EnumParam, FloatParam, FloatRange, Params, SmoothingStyle,
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
}

impl From<LfoType> for Waveform {
    fn from(t: LfoType) -> Self {
        match t {
            LfoType::Sine => Waveform::Sine,
            LfoType::Square => Waveform::Square,
            LfoType::Random => Waveform::Random,
        }
    }
}

#[derive(Params)]
pub struct TremoloParams {
    #[id = "gain"]
    pub gain: FloatParam,
    #[id = "depth"] 
    pub depth: FloatParam,
    #[id = "rate"]
    pub rate: FloatParam,
    #[id = "lfo"]
    pub lfo: EnumParam<LfoType>,
}

impl TremoloParams {
    pub fn new() -> Self {
        Self {
            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                }
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            depth: FloatParam::new(
                "Depth",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 }
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" %")
            .with_value_to_string(formatters::v2s_f32_percentage(2))
            .with_string_to_value(formatters::s2v_f32_percentage()),
            rate: FloatParam::new(
                "Rate",
                600.0,
                FloatRange::Skewed {
                    min: 0.01,
                    max: 32.0,
                    factor: FloatRange::skew_factor(-2.0),
                }
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            // .with_unit(" Hz")
            .with_value_to_string(formatters::v2s_f32_hz_then_khz(2))
            .with_string_to_value(formatters::s2v_f32_hz_then_khz()),
            lfo: EnumParam::new("LFO", LfoType::Sine),
        }
    }
}