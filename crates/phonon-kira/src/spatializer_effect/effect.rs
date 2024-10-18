use super::builder::DirectEffectBuilder;
use super::CommandReaders;
use kira::clock::clock_info::ClockInfoProvider;
use kira::effect::Effect;
use kira::modulator::value_provider::ModulatorValueProvider;
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
    current_sample: usize,
    frame_count: usize,
}

impl DirectEffectWrapped {
    pub(crate) fn new(builder: DirectEffectBuilder, command_readers: CommandReaders) -> Self {
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
            current_sample: 0,
            frame_count: 0,
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

    fn process(
        &mut self,
        input: Frame,
        _dt: f64,
        _clock_info_provider: &ClockInfoProvider,
        _modulator_value_provider: &ModulatorValueProvider,
    ) -> Frame {
        self.audio_buffer[0][self.current_sample] = input.left;
        self.audio_buffer[1][self.current_sample] = input.right;

        let output_sample_l = self.output_buffer[0][self.current_sample];
        let output_sample_r = self.output_buffer[1][self.current_sample];

        if self.current_sample < self.direct_effect.frame_size - 1 {
            self.current_sample += 1;
        } else {
            self.audio_buffer.downmix(&mut self.mono_buffer0);

            self.direct_effect
                .apply(self.parameters, &self.mono_buffer0, &mut self.mono_buffer1);

            self.binaural_effect.apply(
                self.binaural_params,
                &self.mono_buffer1,
                &mut self.output_buffer,
            );

            self.current_sample = 0;
            self.frame_count += 1;
        }

        Frame {
            left: output_sample_l,
            right: output_sample_r,
        }
    }
}
