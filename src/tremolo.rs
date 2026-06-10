use std::sync::Arc;
use std::sync::atomic::Ordering::{Relaxed};
use nice_plug::prelude::*;
use crate::params::{TremoloParams, TremoloMode};
use crate::lfo::{Lfo, Waveform};
use crate::crossover::Biquad;

pub struct Tremolo {
    pub params: Arc<TremoloParams>,
    pub lfo: Lfo,
    pub sample_rate: Arc<AtomicF32>,
    pub lpf: [Biquad; 2],
}

impl Default for Tremolo {
    fn default() -> Self {
        let sample_rate = Arc::new(AtomicF32::new(44100.0));
        Self {
            params: Arc::new(TremoloParams::new()),
            lfo: Lfo::new(sample_rate.load(Relaxed), 4.0, Waveform::Sine),
            sample_rate,
            lpf: [Biquad::default(), Biquad::default()],
        }
    }
}

impl Tremolo {
    fn modulation_gain(lfo_value: f32, depth: f32) -> f32 {
        let mod_amount = (lfo_value + 1.0) * 0.5;
        1.0 - depth + depth * mod_amount
    }

    /// Advance modulators once per audio sample frame.
    pub fn next_modulation(&mut self) -> (f32, f32) {
        self.lfo.set_rate_hz(self.params.rate.smoothed.next());
        self.lfo.set_waveform(self.params.lfo.value().into());
        self.lfo.set_flip_probability(self.params.flip_probability.smoothed.next());
        self.lfo.set_bias(self.params.bias.smoothed.next());

        self.lfo.update_phase();
        let depth = self.params.depth.smoothed.next();

        match self.params.mode.value() {
            TremoloMode::Classic => {
                let gain = Self::modulation_gain(self.lfo.waveform_at(self.lfo.phase), depth);
                (gain, 0.0)
            }
            TremoloMode::Harmonic => {
                let lfo_low = self.lfo.waveform_at(self.lfo.phase);
                let lfo_high = self.lfo.waveform_at(self.lfo.phase + 0.5);
                (
                    Self::modulation_gain(lfo_low, depth),
                    Self::modulation_gain(lfo_high, depth),
                )
            }
        }
    }

    pub fn apply_classic(&self, sample: f32, gain: f32) -> f32 {
        sample * gain
    }

    pub fn apply_harmonic(&mut self, sample: f32, gain_low: f32, gain_high: f32, channel: usize) -> f32 {
        let lpf = &mut self.lpf[channel.min(1)];
        let low = lpf.process(sample);
        let high = sample - low;
        low * gain_low + high * gain_high
    }
}
