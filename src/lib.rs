use nice_plug::prelude::*;
use std::sync::Arc;
use std::sync::atomic::Ordering::{Relaxed};

use crossover::FilterType;
use params::TremoloMode;

mod editor;
mod lfo;
mod params;
mod tremolo;
mod crossover;

impl Plugin for tremolo::Tremolo {
    const NAME: &'static str = "seiche";
    const VENDOR: &'static str = "";
    const URL: &'static str = "";
    const EMAIL: &'static str = "";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            aux_input_ports: &[],
            aux_output_ports: &[],
            names: PortNames::const_default(),
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.sample_rate.store(buffer_config.sample_rate, Relaxed);
        self.lfo.sample_rate = buffer_config.sample_rate;
        for lpf in &mut self.lpf {
            lpf.coefficients
                .update(400.0, buffer_config.sample_rate, FilterType::Lowpass);
        }
        true
    }

    fn reset(&mut self) {
        self.lfo.reset();
        let sample_rate = self.sample_rate.load(Relaxed);
        for lpf in &mut self.lpf {
            lpf.reset_state();
            lpf.coefficients
                .update(400.0, sample_rate, FilterType::Lowpass);
        }
    }

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // Update the tempo from the transport if it's available
        let transport = _context.transport();
        if let Some(tempo) = transport.tempo {
            self.tempo.store(tempo as f32, Relaxed);
        }

        for channel_samples in buffer.iter_samples() {
            if self.params.crossover_frequency.smoothed.is_smoothing() {
                let crossover_frequency = self.params.crossover_frequency.smoothed.next();
                for lpf in &mut self.lpf {
                    lpf.coefficients
                        .update(crossover_frequency, self.sample_rate.load(Relaxed), FilterType::Lowpass);
                }
            }

            let (gain_low, gain_high) = self.next_modulation();
            for (channel, sample) in channel_samples.into_iter().enumerate() {
                *sample = match self.params.mode.value() {
                    TremoloMode::Classic => self.apply_classic(*sample, gain_low),
                    TremoloMode::Harmonic => {
                        self.apply_harmonic(*sample, gain_low, gain_high, channel)
                    }
                };
            }
        }
        ProcessStatus::Normal
    }
}

impl ClapPlugin for tremolo::Tremolo {
    const CLAP_ID: &'static str = "info.bjustice.seiche";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Seiche");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Mono,
    ];
}

impl Vst3Plugin for tremolo::Tremolo {
    const VST3_CLASS_ID: [u8; 16] = *b"seicheeeeeeeeeee";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx];
}

nice_export_clap!(tremolo::Tremolo);
nice_export_vst3!(tremolo::Tremolo);
