use kira::clock::clock_info::ClockInfoProvider;
use kira::dsp::Frame;
use kira::manager::backend::cpal::CpalBackend;
use kira::manager::{AudioManager, AudioManagerSettings};
use kira::modulator::value_provider::ModulatorValueProvider;
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use kira::track::effect::Effect;
use kira::track::effect::filter::{FilterBuilder, FilterMode};
use kira::track::TrackBuilder;
use kira::tween::Parameter;
use phonon::audio_buffer::AudioSettings;
use phonon::eq_effect::EqEffect;
//use ringbuf::HeapConsumer;

struct EqEffectWrapped {
    eq_effect: EqEffect,
    //command_consumer: HeapConsumer<Command>,
    ic1eq: Frame,
    ic2eq: Frame,
}

impl Effect for EqEffectWrapped {
    fn on_start_processing(&mut self) {
        todo!()
    }

    fn process(&mut self, input: Frame, dt: f64, clock_info_provider: &ClockInfoProvider, modulator_value_provider: &ModulatorValueProvider) -> Frame {
        todo!()
    }
}

fn main() {
    let audio_settings = AudioSettings::new(44_100, 1024);
    let eq_effect = EqEffect::new(audio_settings);
    let eq_gains: [f32; 3] = [1.0, 1.0, 1.0];

    let mut manager = AudioManager::<CpalBackend>::new(AudioManagerSettings::default()).unwrap();
    let track = manager
        .add_sub_track(TrackBuilder::new().with_effect(FilterBuilder::new().cutoff(1000.0)))
        .unwrap();
    let sound_data = StaticSoundData::from_file(
        "data/audio/pink_noise.ogg",
        StaticSoundSettings::new().output_destination(&track),
    )
    .unwrap();

    manager.play(sound_data).unwrap();

    loop {}
}
