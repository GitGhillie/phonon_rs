use super::builder::EqEffectBuilder;
use super::CommandReaders;
use kira::effect::Effect;
use kira::info::Info;
use kira::Frame;
use phonon::dsp::audio_buffer::{AudioBuffer, AudioSettings};
use phonon::effects::eq::{EqEffect, EqEffectParameters};
use phonon::effects::gain::{GainEffect, GainEffectParameters};

pub(crate) struct EqEffectWrapped {
    command_readers: CommandReaders,
    eq_gains: [f32; 3],
    eq_effect: EqEffect,
    gain: f32,
    gain_effect: GainEffect,
    audio_buffer: AudioBuffer<2>,
    mono_buffer: AudioBuffer<1>,
    output_buffer: AudioBuffer<1>,
}

impl EqEffectWrapped {
    pub(crate) fn new(builder: EqEffectBuilder, command_readers: CommandReaders) -> Self {
        let audio_settings = AudioSettings::new(44_100, 1024);
        let eq_effect = EqEffect::new(audio_settings);
        let gain_effect = GainEffect::new(audio_settings);

        Self {
            command_readers,
            eq_gains: builder.eq_gains,
            eq_effect,
            gain: builder.gain,
            gain_effect,
            audio_buffer: AudioBuffer::new(audio_settings.frame_size),
            mono_buffer: AudioBuffer::new(audio_settings.frame_size),
            output_buffer: AudioBuffer::new(audio_settings.frame_size),
        }
    }
}

impl Effect for EqEffectWrapped {
    fn on_start_processing(&mut self) {
        if let Some(command) = self.command_readers.set_eq_gains.read() {
            self.eq_gains = command;
        }
        if let Some(command) = self.command_readers.set_gain.read() {
            self.gain = command;
        }
    }

    fn process(&mut self, mut input: &mut [Frame], _dt: f64, _info: &Info) {
        let (left, right) = input.iter().map(|frame| (frame.left, frame.right)).unzip();

        self.audio_buffer[0] = left;
        self.audio_buffer[1] = right;

        self.audio_buffer.downmix(&mut self.output_buffer);

        self.gain_effect.apply(
            GainEffectParameters { gain: self.gain },
            &self.output_buffer,
            &mut self.mono_buffer,
        );

        self.eq_effect.apply(
            EqEffectParameters {
                gains: self.eq_gains,
            },
            &self.mono_buffer,
            &mut self.output_buffer,
        );

        // todo avoid clone here?
        input = self.output_buffer[0]
            .iter()
            .cloned()
            .map(|sample| Frame::new(sample, sample))
            .collect();
    }
}
