use super::builder::ReverbEffectBuilder;
use super::CommandReaders;
use kira::effect::Effect;
use kira::info::Info;
use kira::Frame;
use phonon::dsp::audio_buffer::{AudioBuffer, AudioSettings};
use phonon::dsp::reverb_estimator::Reverb;
use phonon::effects::reverb::{ReverbEffect, ReverbEffectParams};

pub(crate) struct ReverbEffectWrapped {
    command_readers: CommandReaders,
    reverb_times: [f32; 3],
    reverb_effect: ReverbEffect,
    dry: bool,
    wet: bool,
    audio_buffer: AudioBuffer<2>,
    result_buffer: AudioBuffer<1>,
    mono_buffer: AudioBuffer<1>,
    output_buffer: AudioBuffer<1>,
}

impl ReverbEffectWrapped {
    pub(crate) fn new(builder: ReverbEffectBuilder, command_readers: CommandReaders) -> Self {
        // todo do not guess these values:
        let audio_settings = AudioSettings::new(44_100, 1024);
        let reverb_effect = ReverbEffect::new(audio_settings);

        Self {
            command_readers,
            reverb_times: builder.reverb_times,
            reverb_effect,
            dry: builder.dry,
            wet: builder.wet,
            audio_buffer: AudioBuffer::new(audio_settings.frame_size),
            result_buffer: AudioBuffer::new(audio_settings.frame_size),
            mono_buffer: AudioBuffer::new(audio_settings.frame_size),
            output_buffer: AudioBuffer::new(audio_settings.frame_size),
        }
    }
}

impl Effect for ReverbEffectWrapped {
    fn on_start_processing(&mut self) {
        if let Some(command) = self.command_readers.set_reverb_times.read() {
            self.reverb_times = command;
        }
        if let Some(command) = self.command_readers.set_dry.read() {
            self.dry = command;
        }
        if let Some(command) = self.command_readers.set_wet.read() {
            self.wet = command;
        }
    }

    // todo: process tail
    fn process(&mut self, mut input: &mut [Frame], _dt: f64, _info: &Info) {
        // De-interlace:
        let (left, right) = input.iter().map(|frame| (frame.left, frame.right)).unzip();

        self.audio_buffer[0] = left;
        self.audio_buffer[1] = right;

        self.audio_buffer.downmix(&mut self.mono_buffer);

        self.reverb_effect.apply(
            &ReverbEffectParams(Reverb {
                reverb_times: self.reverb_times,
            }),
            &self.mono_buffer,
            &mut self.result_buffer,
        );

        self.output_buffer.make_silent();

        if self.dry {
            for i in 0..self.reverb_effect.frame_size {
                self.output_buffer[0][i] += self.mono_buffer[0][i];
            }
        }

        if self.wet {
            for i in 0..self.reverb_effect.frame_size {
                self.output_buffer[0][i] += self.result_buffer[0][i];
            }
        }

        input.iter_mut().enumerate().for_each(|(i, frame)| {
            frame.left = self.output_buffer[0][i];
            frame.right = self.output_buffer[0][i];
        })
    }
}
