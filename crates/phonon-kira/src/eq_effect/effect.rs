use super::builder::EqEffectBuilder;
use crate::eq_effect::Command;
use kira::clock::clock_info::ClockInfoProvider;
use kira::dsp::Frame;
use kira::modulator::value_provider::ModulatorValueProvider;
use kira::track::effect::Effect;
use phonon::audio_buffer::{AudioBuffer, AudioSettings};
use phonon::eq_effect::{EqEffect, EqEffectParameters};
use ringbuf::HeapConsumer;

pub(crate) struct EqEffectWrapped {
    command_consumer: HeapConsumer<Command>,
    eq_gains: [f32; 3],
    eq_effect: EqEffect,
    audio_buffer: AudioBuffer<2>,
    mono_buffer: AudioBuffer<1>,
    output_buffer: AudioBuffer<1>,
    current_sample: usize,
    frame_count: usize,
}

impl EqEffectWrapped {
    pub(crate) fn new(builder: EqEffectBuilder, command_consumer: HeapConsumer<Command>) -> Self {
        let audio_settings = AudioSettings::new(44_100, 1024);
        let eq_effect = EqEffect::new(audio_settings.clone());

        Self {
            command_consumer,
            eq_gains: builder.eq_gains,
            eq_effect,
            audio_buffer: AudioBuffer::new(audio_settings.frame_size),
            mono_buffer: AudioBuffer::new(audio_settings.frame_size),
            output_buffer: AudioBuffer::new(audio_settings.frame_size),
            current_sample: 0,
            frame_count: 0,
        }
    }
}

impl Effect for EqEffectWrapped {
    fn on_start_processing(&mut self) {
        while let Some(command) = self.command_consumer.pop() {
            match command {
                Command::SetEqGains(gains) => self.eq_gains = gains,
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

        if self.current_sample < self.eq_effect.frame_size - 1 {
            self.current_sample += 1;
        } else {
            self.audio_buffer.downmix(&mut self.mono_buffer);

            self.eq_effect.apply(
                EqEffectParameters {
                    gains: self.eq_gains,
                },
                &self.mono_buffer,
                &mut self.output_buffer,
            );

            self.current_sample = 0;
            self.frame_count += 1;
        }

        Frame {
            left: output_sample,
            right: output_sample,
        }
    }
}
