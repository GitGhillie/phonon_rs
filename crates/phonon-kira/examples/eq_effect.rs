use kira::manager::backend::cpal::CpalBackend;
use kira::manager::{AudioManager, AudioManagerSettings};
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use kira::track::TrackBuilder;
use phonon_kira::EqEffectBuilder;

fn main() {
    let eq_gains: [f32; 3] = [1.0, 0.5, 1.0];

    let mut manager = AudioManager::<CpalBackend>::new(AudioManagerSettings::default()).unwrap();

    let track = manager
        .add_sub_track(TrackBuilder::new().with_effect(EqEffectBuilder { eq_gains }))
        .unwrap();

    let sound_data = StaticSoundData::from_file(
        "data/audio/pink_noise.ogg",
        StaticSoundSettings::new().output_destination(&track),
    )
    .unwrap();

    manager.play(sound_data).unwrap();

    loop {}
}
