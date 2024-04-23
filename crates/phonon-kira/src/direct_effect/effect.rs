use super::builder::DirectEffectBuilder;
use super::Command;
use kira::clock::clock_info::ClockInfoProvider;
use kira::dsp::Frame;
use kira::modulator::value_provider::ModulatorValueProvider;
use kira::track::effect::Effect;
use phonon::audio_buffer::{AudioBuffer, AudioSettings};
use phonon::direct_effect::{DirectEffect, DirectEffectParameters};
use ringbuf::HeapConsumer;

pub(crate) struct DirectEffectWrapped {
    command_consumer: HeapConsumer<Command>,
    direct_effect: DirectEffect,
    parameters: DirectEffectParameters,
    audio_buffer: AudioBuffer<2>,
    mono_buffer: AudioBuffer<1>,
    output_buffer: AudioBuffer<1>,
    current_sample: usize,
    frame_count: usize,
}

impl DirectEffectWrapped {
    pub(crate) fn new(
        builder: DirectEffectBuilder,
        command_consumer: HeapConsumer<Command>,
    ) -> Self {
        let audio_settings = AudioSettings::new(44_100, 1024);
        let direct_effect = DirectEffect::new(audio_settings.clone());

        Self {
            command_consumer,
            direct_effect,
            parameters: builder.parameters,
            audio_buffer: AudioBuffer::new(audio_settings.frame_size),
            mono_buffer: AudioBuffer::new(audio_settings.frame_size),
            output_buffer: AudioBuffer::new(audio_settings.frame_size),
            current_sample: 0,
            frame_count: 0,
        }
    }
}

impl Effect for DirectEffectWrapped {
    fn on_start_processing(&mut self) {
        while let Some(command) = self.command_consumer.pop() {
            match command {
                Command::SetParameters(params) => self.parameters = params,
            }
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

        let output_sample = self.output_buffer[0][self.current_sample];

        if self.current_sample < self.direct_effect.frame_size - 1 {
            self.current_sample += 1;
        } else {
            self.audio_buffer.downmix(&mut self.mono_buffer);

            self.direct_effect
                .apply(self.parameters, &self.mono_buffer, &mut self.output_buffer);

            self.current_sample = 0;
            self.frame_count += 1;
        }

        Frame {
            left: output_sample,
            right: output_sample,
        }
    }
}
