use std::sync::Arc;
use std::sync::atomic::Ordering::{Relaxed};
use crate::params::{TremoloParams, TremoloMode, RateMode};
use crate::lfo::{Lfo, Waveform};
use crate::crossover::Biquad;
use atomic_float::AtomicF32;
pub struct Tremolo {
    pub params: Arc<TremoloParams>,
    pub lfo: Lfo,
    pub sample_rate: Arc<AtomicF32>,
    pub lpf: [Biquad; 2],
    pub tempo: Arc<AtomicF32>,
}

impl Default for Tremolo {
    fn default() -> Self {
        let sample_rate = Arc::new(AtomicF32::new(44100.0));

        Self {
            params: Arc::new(TremoloParams::new()),
            lfo: Lfo::new(sample_rate.load(Relaxed), 4.0, Waveform::Sine),
            sample_rate,
            lpf: [Biquad::default(), Biquad::default()],
            tempo: Arc::new(AtomicF32::new(120.0)),
        }
    }
}

impl Tremolo {

    /// Set the rate based on the rate mode and sync division
    /// @param free_rate: The free rate to set if the rate mode is free
    /// @return The rate to set in Hz
    fn set_rate_hz(&self, free_rate: f32) -> f32 {
        match self.params.rate_mode.value() {
            RateMode::Free => free_rate,
            RateMode::Sync => {
                let bpm = self.tempo.load(Relaxed);
                let beats = self.params.sync_division.value().beats_per_cycle();
                // At 120 BPM, quarter-note sync (beats = 1.0) → 2 Hz. Eighth-note → 4 Hz.
                (bpm / 60.0) / beats
            }
        }
    }

    fn modulation_gain(lfo_value: f32, depth: f32) -> f32 {
        let mod_amount = (lfo_value + 1.0) * 0.5; // maps lfo_value from [0..1]
        1.0 - depth + depth * mod_amount
    }

    /// Advance modulators once per audio sample frame.
    pub fn next_modulation(&mut self) -> (f32, f32) {
        self.lfo.set_waveform(self.params.lfo.value().into());
        self.lfo.set_flip_probability(self.params.flip_probability.smoothed.next());
        self.lfo.set_bias(self.params.bias.smoothed.next());
        
        self.lfo.update_phase();
        let depth = self.params.depth.smoothed.next();
        
        // Set the rate based on the rate mode and sync division
        let free_rate = self.params.rate.smoothed.next();
        self.lfo.set_rate_hz(self.set_rate_hz(free_rate));

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