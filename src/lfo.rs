use std::f32::consts::TAU;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Waveform {
    Sine,
    Square,
    Random,
}

pub struct Lfo {
    pub sample_rate: f32,
    pub rate_hz: f32,
    pub waveform: Waveform,
    pub phase: f32,
    pub random_hold: f32,
    pub rng_state: u32,
}

impl Lfo {
    pub fn new(sample_rate: f32, rate_hz: f32, waveform: Waveform) -> Self {
        Self {
            sample_rate,
            rate_hz,
            waveform,
            phase: 0.0,
            random_hold: 0.0,
            rng_state: 0,
        }
    }

    pub fn set_rate_hz(&mut self, rate_hz: f32) {
        self.rate_hz = rate_hz;
    }

    pub fn set_waveform(&mut self, waveform: Waveform) {
        self.waveform = waveform;
    }

    pub fn next(&mut self) -> f32 {
        let value = match self.waveform {
            Waveform::Sine => (self.phase * TAU).sin(),
            Waveform::Square => if self.phase < 0.5 { 1.0 } else { -1.0 },
            Waveform::Random => self.random_hold * 2.0 - 1.0,
        };
        self.update_phase();
        value
    }

    pub fn reset(&mut self) {
        self.phase = 0.0;
        self.random_hold = 0.0;
        self.rng_state = 0;
    }

    fn update_phase(&mut self) {
        self.phase += self.rate_hz / self.sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        if self.waveform == Waveform::Random {
            self.random_hold = self.next_random_unit();
        }
    }

    fn next_random_unit(&mut self) -> f32 {
        // e.g. park-miller style LCG
        self.rng_state = self.rng_state
            .wrapping_mul(1_664_525)
            .wrapping_add(1_013_904_223);
        (self.rng_state >> 8) as f32 / 16_777_216.0  // [0, 1)
    }
}