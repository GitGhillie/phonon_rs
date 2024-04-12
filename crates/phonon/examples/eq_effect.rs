use kira::clock::clock_info::ClockInfoProvider;
use kira::dsp::Frame;
use kira::manager::backend::cpal::CpalBackend;
use kira::manager::{AudioManager, AudioManagerSettings};
use kira::modulator::value_provider::ModulatorValueProvider;
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use kira::track::effect::{Effect, EffectBuilder};
use kira::track::TrackBuilder;
use phonon::audio_buffer::{AudioBuffer, AudioSettings};
use phonon::eq_effect::{EqEffect, EqEffectParameters};

#[derive(Debug, Copy, Clone, PartialEq)]
struct EqEffectBuilder {
    eq_gains: [f32; 3],
}

impl EffectBuilder for EqEffectBuilder {
    type Handle = ();

    fn build(self) -> (Box<dyn Effect>, Self::Handle) {
        (Box::new(EqEffectWrapped::new(self)), ())
    }
}

struct EqEffectWrapped {
    eq_gains: [f32; 3],
    eq_effect: EqEffect,
    audio_buffer: AudioBuffer<1>,
    output_buffer: AudioBuffer<1>,
    current_sample: usize,
    frame_count: usize,
}

impl EqEffectWrapped {
    fn new(builder: EqEffectBuilder) -> Self {
        let audio_settings = AudioSettings::new(44_100, 1024);
        let eq_effect = EqEffect::new(audio_settings.clone());

        Self {
            eq_gains: builder.eq_gains,
            eq_effect,
            audio_buffer: AudioBuffer::new(audio_settings.frame_size),
            output_buffer: AudioBuffer::new(audio_settings.frame_size),
            current_sample: 0,
            frame_count: 0,
        }
    }
}

impl Effect for EqEffectWrapped {
    fn process(
        &mut self,
        input: Frame,
        _dt: f64,
        _clock_info_provider: &ClockInfoProvider,
        _modulator_value_provider: &ModulatorValueProvider,
    ) -> Frame {
        let mut output_sample = 0.0;

        if input.left > 0.0 {
            output_sample = 0.1;
        }

        // todo: downmix to mono instead of taking one channel
        self.audio_buffer[0][self.current_sample] = input.left;
        output_sample = self.output_buffer[0][self.current_sample];

        if self.current_sample < self.eq_effect.frame_size - 1 {
            self.current_sample += 1;
        } else {
            self.eq_effect.apply(
                EqEffectParameters {
                    gains: self.eq_gains,
                },
                &self.audio_buffer,
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

fn main() {
    let eq_gains: [f32; 3] = [1.0, 1.0, 1.0];

    let mut manager = AudioManager::<CpalBackend>::new(AudioManagerSettings::default()).unwrap();

    let track = manager
        .add_sub_track(TrackBuilder::new().with_effect(EqEffectBuilder { eq_gains }))
        .unwrap();

    let sound_data = StaticSoundData::from_file(
        "data/audio/return_solo.mp3",
        StaticSoundSettings::new().output_destination(&track),
    )
    .unwrap();

    manager.play(sound_data).unwrap();

    loop {}
}
