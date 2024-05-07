use super::builder::DirectEffectBuilder;
use super::Command;
use kira::clock::clock_info::ClockInfoProvider;
use kira::dsp::Frame;
use kira::modulator::value_provider::ModulatorValueProvider;
use kira::track::effect::Effect;
use phonon::audio_buffer::{AudioBuffer, AudioSettings};
use phonon::direct_effect::{DirectEffect, DirectEffectParameters};
use phonon::panning_effect::{PanningEffect, PanningEffectParameters};
use phonon::speaker_layout::SpeakerLayoutType;
use ringbuf::HeapConsumer;

pub(crate) struct DirectEffectWrapped {
    command_consumer: HeapConsumer<Command>,
    direct_effect: DirectEffect,
    parameters: DirectEffectParameters,
    panning_effect: PanningEffect,
    panning_params: PanningEffectParameters,
    audio_buffer: AudioBuffer<2>,
    mono_buffer0: AudioBuffer<1>,
    mono_buffer1: AudioBuffer<1>,
    output_buffer: AudioBuffer<2>,
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
        let panning_effect = PanningEffect::new(SpeakerLayoutType::Stereo);

        Self {
            command_consumer,
            direct_effect,
            parameters: builder.parameters,
            panning_effect,
            panning_params: builder.panning_params,
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
        while let Some(command) = self.command_consumer.pop() {
            match command {
                Command::SetParameters(params) => self.parameters = params,
                Command::SetPanning(params) => self.panning_params = params,
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

        let output_sample_l = self.output_buffer[0][self.current_sample];
        let output_sample_r = self.output_buffer[1][self.current_sample];

        if self.current_sample < self.direct_effect.frame_size - 1 {
            self.current_sample += 1;
        } else {
            self.audio_buffer.downmix(&mut self.mono_buffer0);

            self.direct_effect
                .apply(self.parameters, &self.mono_buffer0, &mut self.mono_buffer1);

            self.panning_effect.apply(
                self.panning_params,
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
