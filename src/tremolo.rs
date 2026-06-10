use std::sync::Arc;
use std::sync::atomic::Ordering::{Relaxed};
use nice_plug::prelude::*;
use crate::params::{TremoloParams};
use crate::lfo::{Lfo, Waveform};

pub struct Tremolo {
    pub params: Arc<TremoloParams>,
    pub lfo: Lfo,
    pub sample_rate: Arc<AtomicF32>,
}

impl Default for Tremolo {
    fn default() -> Self {
        let sample_rate = Arc::new(AtomicF32::new(44100.0));
        Self {
            params: Arc::new(TremoloParams::new()),
            lfo: Lfo::new(sample_rate.load(Relaxed), 4.0, Waveform::Sine),
            sample_rate,
        }
    }
}

impl Tremolo {
    /// Advance modulators once per audio sample frame. Call this once per `iter_samples()` step,
    /// then apply the returned gain to each channel.
    pub fn next_modulation(&mut self) -> f32 {
        self.lfo.set_rate_hz(self.params.rate.smoothed.next());
        self.lfo.set_waveform(self.params.lfo.value().into());
        self.lfo.set_flip_probability(self.params.flip_probability.smoothed.next());
        self.lfo.set_bias(self.params.bias.smoothed.next());

        let lfo_value = self.lfo.next();
        let depth = self.params.depth.smoothed.next();

        let mod_amount = (lfo_value + 1.0) * 0.5; // map [-1, 1] to [0, 1]
        let tremolo_depth = 1.0 - depth + depth * mod_amount;

        tremolo_depth
    }

    pub fn apply_modulation(&self, sample: f32, modulation: f32) -> f32 {
        sample * modulation
    }
}