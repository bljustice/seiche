use std::f32::consts::TAU;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Waveform {
    Sine,
    Square,
    Random,
    Ramp,
}

pub struct Lfo {
    pub sample_rate: f32,
    pub rate_hz: f32,
    pub waveform: Waveform,
    pub phase: f32,
    /// Bipolar output in [-1, 1], used by the tremolo modulator.
    pub random_value: f32,
    random_start: f32,
    random_target: f32,
    /// Chance to invert the ±1 step when picking a new target each cycle.
    pub flip_probability: f32,
    /// Constant drift added to each step before it becomes the next target.
    pub bias: f32,
    pub rng_state: u32,
}

impl Lfo {
    pub fn new(sample_rate: f32, rate_hz: f32, waveform: Waveform) -> Self {
        Self {
            sample_rate,
            rate_hz,
            waveform,
            phase: 0.0,
            random_value: 0.0,
            random_start: 0.0,
            random_target: 0.0,
            flip_probability: 0.3,
            bias: 0.2,
            rng_state: 1,
        }
    }

    pub fn set_flip_probability(&mut self, flip_probability: f32) {
        self.flip_probability = flip_probability;
    }

    pub fn set_bias(&mut self, bias: f32) {
        self.bias = bias;
    }

    pub fn set_rate_hz(&mut self, rate_hz: f32) {
        self.rate_hz = rate_hz;
    }

    pub fn set_waveform(&mut self, waveform: Waveform) {
        if waveform == Waveform::Random && self.waveform != Waveform::Random {
            self.random_start = self.random_value;
            self.random_target = self.next_random_target();
        }
        self.waveform = waveform;
    }
    
    pub fn waveform_at(&self, mut phase: f32) -> f32 {
        phase -= phase.floor(); // wrap to [0, 1)
        match self.waveform {
            Waveform::Sine => (phase * TAU).sin(),
            Waveform::Square => soft_square(phase, self.rate_hz),
            Waveform::Random => self.random_at_phase(phase),
            Waveform::Ramp => soft_ramp(phase, self.rate_hz),
        }
    }

    fn random_at_phase(&self, phase: f32) -> f32 {
        let t = smoothstep(phase);
        self.random_start + (self.random_target - self.random_start) * t
    }

    pub fn reset(&mut self) {
        self.phase = 0.0;
        self.random_value = 0.0;
        self.random_start = 0.0;
        self.random_target = 0.0;
        self.rng_state = 1;
    }

    pub fn update_phase(&mut self) {
        self.phase += self.rate_hz / self.sample_rate;
        let phase_wrapped = self.phase >= 1.0;
        if phase_wrapped {
            self.phase -= 1.0;
        }

        if self.waveform == Waveform::Random {
            self.update_random(phase_wrapped);
        }
    }

    /// Rate-synced random LFO: pick a new target each cycle, smoothstep between targets.
    fn update_random(&mut self, phase_wrapped: bool) {
        if phase_wrapped {
            self.random_start = self.random_target;
            self.random_target = self.next_random_target();
        }

        let t = smoothstep(self.phase);
        self.random_value =
            self.random_start + (self.random_target - self.random_start) * t;
    }

    fn next_random_target(&mut self) -> f32 {
        let mut step = if self.next_random_unit() >= 0.5 { 1.0 } else { -1.0 };

        if self.next_random_unit() < self.flip_probability.clamp(0.0, 1.0) {
            step = -step;
        }

        step += self.bias;

        (self.random_target + step * 0.5).clamp(-1.0, 1.0)
    }

    fn next_random_unit(&mut self) -> f32 {
        self.rng_state = self
            .rng_state
            .wrapping_mul(1_664_525)
            .wrapping_add(1_013_904_223);
        (self.rng_state >> 8) as f32 / 16_777_216.0
    }
}

fn soft_square(phase: f32, rate_hz: f32) -> f32 {
    let edge = (0.002 * rate_hz).clamp(0.003, 0.1);

    if phase < 0.5 - edge {
        1.0
    } else if phase < 0.5 + edge {
        let t = (phase - (0.5 - edge)) / (2.0 * edge);
        1.0 - 2.0 * smoothstep(t)
    } else if phase < 1.0 - edge {
        -1.0
    } else {
        let t = (phase - (1.0 - edge)) / edge;
        -1.0 + 2.0 * smoothstep(t)
    }
}

fn soft_ramp(phase: f32, rate_hz: f32) -> f32 {
    let edge = (0.002 * rate_hz).clamp(0.003, 0.1);
    let unipolar = if phase < edge {
        let t = phase / edge;
        smoothstep(t) * (edge / (1.0 - edge))
    } else if phase < 1.0 - edge {
        phase / (1.0 - edge)
    } else {
        let t = (phase - (1.0 - edge)) / edge;
        1.0 - smoothstep(t)
    };

    1.0 - 2.0 * unipolar.powf(2.5)
}




fn smoothstep(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

