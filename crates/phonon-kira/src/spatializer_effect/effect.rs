use super::builder::DirectEffectBuilder;
use super::CommandReaders;
use kira::effect::Effect;
use kira::info::Info;
use kira::Frame;
use phonon::dsp::audio_buffer::{AudioBuffer, AudioSettings};
use phonon::effects::binaural::{BinauralEffect, BinauralEffectParameters};
use phonon::effects::direct::{DirectEffect, DirectEffectParameters};

pub(crate) struct DirectEffectWrapped {
    command_readers: CommandReaders,
    direct_effect: DirectEffect,
    parameters: DirectEffectParameters,
    binaural_effect: BinauralEffect,
    binaural_params: BinauralEffectParameters,
    audio_buffer: AudioBuffer<2>,
    mono_buffer0: AudioBuffer<1>,
    mono_buffer1: AudioBuffer<1>,
    output_buffer: AudioBuffer<2>,
}

impl DirectEffectWrapped {
    pub(crate) fn new(builder: DirectEffectBuilder, command_readers: CommandReaders) -> Self {
        // todo do not guess these values:
        let audio_settings = AudioSettings::new(44_100, 1024);
        let direct_effect = DirectEffect::new(audio_settings);
        let binaural_effect = BinauralEffect::new(audio_settings);

        Self {
            command_readers,
            direct_effect,
            parameters: builder.parameters,
            binaural_effect,
            binaural_params: builder.binaural_params,
            audio_buffer: AudioBuffer::new(audio_settings.frame_size),
            mono_buffer0: AudioBuffer::new(audio_settings.frame_size),
            mono_buffer1: AudioBuffer::new(audio_settings.frame_size),
            output_buffer: AudioBuffer::new(audio_settings.frame_size),
        }
    }
}

impl Effect for DirectEffectWrapped {
    fn on_start_processing(&mut self) {
        if let Some(command) = self.command_readers.set_parameters.read() {
            self.parameters = command;
        }
        if let Some(command) = self.command_readers.set_direction.read() {
            self.binaural_params = command;
        }
    }

    fn process(&mut self, mut input: &mut [Frame], _dt: f64, _info: &Info) {
        // De-interlace:
        let (left, right) = input.iter().map(|frame| (frame.left, frame.right)).unzip();

        self.audio_buffer[0] = left;
        self.audio_buffer[1] = right;

        self.audio_buffer.downmix(&mut self.mono_buffer0);

        self.direct_effect
            .apply(self.parameters, &self.mono_buffer0, &mut self.mono_buffer1);

        self.binaural_effect.apply(
            self.binaural_params,
            &self.mono_buffer1,
            &mut self.output_buffer,
        );

        input.iter_mut().enumerate().for_each(|(i, frame)| {
            frame.left = self.output_buffer[0][i];
            frame.right = self.output_buffer[1][i];
        })
    }
}
