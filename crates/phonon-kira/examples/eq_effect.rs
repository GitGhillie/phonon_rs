use eframe::egui;

use kira::manager::backend::cpal::CpalBackend;
use kira::manager::{AudioManager, AudioManagerSettings};
use kira::sound::static_sound::{StaticSoundData, StaticSoundSettings};
use kira::track::TrackBuilder;

use phonon_kira::eq_effect::builder::EqEffectBuilder;

fn main() {
    let mut eq_gains: [f32; 3] = [1.0, 1.0, 1.0];
    let mut gain: f32 = 1.0;

    let mut manager = AudioManager::<CpalBackend>::new(AudioManagerSettings::default()).unwrap();

    let mut track_builder = TrackBuilder::new();
    let mut effect_handle = track_builder.add_effect(EqEffectBuilder { eq_gains, gain });
    let track = manager.add_sub_track(track_builder).unwrap();

    let sound_data = StaticSoundData::from_file(
        "data/audio/pink_noise.ogg",
        StaticSoundSettings::new()
            .loop_region(..)
            .output_destination(&track),
    )
    .unwrap();

    manager.play(sound_data).unwrap();

    eframe::run_simple_native(
        "EQ & Gain Effect (Kira)",
        eframe::NativeOptions::default(),
        move |ctx, _frame| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.add(egui::Slider::new(&mut gain, 0.0..=1.0).text("Gain"));
                ui.add(egui::Slider::new(&mut eq_gains[0], 0.0..=1.0).text("Gain Low"));
                ui.add(egui::Slider::new(&mut eq_gains[1], 0.0..=1.0).text("Gain Mid"));
                ui.add(egui::Slider::new(&mut eq_gains[2], 0.0..=1.0).text("Gain High"));
            });

            effect_handle.set_eq_gains(eq_gains).unwrap();
            effect_handle.set_gain(gain).unwrap();
        },
    )
    .unwrap()
}
