use eframe::egui;

use kira::manager::backend::cpal::CpalBackend;
use kira::manager::{AudioManager, AudioManagerSettings};
use kira::sound::static_sound::StaticSoundData;
use kira::track::TrackBuilder;
use phonon::dsp::reverb_estimator::Reverb;
use phonon::effects::reverb_effect::ReverbEffectParams;

use phonon_kira::reverb_effect::builder::ReverbEffectBuilder;

fn main() {
    let mut reverb_params = ReverbEffectParams(Reverb {
        reverb_times: [2.0, 1.5, 1.0],
    });

    let mut dry = false;
    let mut wet = true;

    let mut manager = AudioManager::<CpalBackend>::new(AudioManagerSettings::default()).unwrap();

    let mut track_builder = TrackBuilder::new();
    let mut effect_handle = track_builder.add_effect(ReverbEffectBuilder {
        reverb_times: reverb_params.reverb_times,
        dry,
        wet,
    });
    let track = manager.add_sub_track(track_builder).unwrap();

    let sound_data = StaticSoundData::from_file("data/audio/windless_slopes.ogg")
        .unwrap()
        .loop_region(..)
        .output_destination(&track);

    manager.play(sound_data).unwrap();

    eframe::run_simple_native(
        "Direct Sound Effect (Kira)",
        eframe::NativeOptions::default(),
        move |ctx, _frame| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.add(
                    egui::Slider::new(&mut reverb_params.reverb_times[0], 0.1..=10.0)
                        .text("Reverb Low"),
                );

                ui.add(
                    egui::Slider::new(&mut reverb_params.reverb_times[1], 0.1..=10.0)
                        .text("Reverb Mid"),
                );

                ui.add(
                    egui::Slider::new(&mut reverb_params.reverb_times[2], 0.1..=10.0)
                        .text("Reverb High"),
                );

                ui.add(egui::Checkbox::new(&mut dry, "Dry"));

                ui.add(egui::Checkbox::new(&mut wet, "Wet"));
            });

            effect_handle.set_reverb_times(reverb_params.reverb_times);
            effect_handle.set_dry(dry);
            effect_handle.set_wet(wet);
        },
    )
    .unwrap()
}
