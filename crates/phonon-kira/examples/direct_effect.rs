use eframe::egui;

use kira::manager::backend::cpal::CpalBackend;
use kira::manager::{AudioManager, AudioManagerSettings};
use kira::sound::static_sound::StaticSoundData;
use kira::track::TrackBuilder;
use phonon::direct_effect::{DirectApplyFlags, DirectEffectParameters, TransmissionType};
use phonon::direct_simulator::DirectSoundPath;

use phonon_kira::direct_effect::builder::DirectEffectBuilder;

fn main() {
    let mut direct_params = DirectEffectParameters {
        direct_sound_path: DirectSoundPath::default(),
        flags: DirectApplyFlags::AirAbsorption | DirectApplyFlags::Occlusion,
        transmission_type: TransmissionType::FrequencyIndependent,
    };

    let mut distance_attenuation = false;

    let mut manager = AudioManager::<CpalBackend>::new(AudioManagerSettings::default()).unwrap();

    let mut track_builder = TrackBuilder::new();
    let mut effect_handle = track_builder.add_effect(DirectEffectBuilder {
        parameters: direct_params,
        panning_params: Default::default(),
    });
    let track = manager.add_sub_track(track_builder).unwrap();

    let sound_data = StaticSoundData::from_file("data/audio/pink_noise.ogg")
        .unwrap()
        .loop_region(..)
        .output_destination(&track);

    manager.play(sound_data).unwrap();

    eframe::run_simple_native(
        "Direct Sound Effect (Kira)",
        eframe::NativeOptions::default(),
        move |ctx, _frame| {
            let sound_path = &mut direct_params.direct_sound_path;

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.add(egui::Checkbox::new(
                    &mut distance_attenuation,
                    "Apply distance attenuation",
                ));

                ui.add(
                    egui::Slider::new(&mut sound_path.distance_attenuation, 0.0..=1.0)
                        .text("Distance Attenuation"),
                );

                ui.label("Air Absorption (AA) parameters:");

                ui.add(
                    egui::Slider::new(&mut sound_path.air_absorption[0], 0.0..=1.0).text("AA Low"),
                );

                ui.add(
                    egui::Slider::new(&mut sound_path.air_absorption[1], 0.0..=1.0).text("AA Mid"),
                );

                ui.add(
                    egui::Slider::new(&mut sound_path.air_absorption[2], 0.0..=1.0).text("AA High"),
                );

                ui.label("");

                ui.add(
                    egui::Slider::new(&mut sound_path.occlusion, 0.0..=1.0)
                        .text("Occlusion factor"),
                );
            });

            direct_params
                .flags
                .set(DirectApplyFlags::DistanceAttenuation, distance_attenuation);

            effect_handle.set_parameters(direct_params);
        },
    )
    .unwrap()
}
