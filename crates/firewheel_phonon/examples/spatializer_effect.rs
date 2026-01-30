//! This example shows how different parameters for the `DirectEffect` affect the sound.
//! Normally the `DirectSoundPath` would be driven by the outputs of the `DirectSimulator`.
//! Additionally, a HRTF is used to create a binaural sound.

use eframe::egui;
use firewheel::FirewheelContext;
use firewheel::diff::Memo;
use firewheel::error::UpdateError;
use firewheel::node::NodeID;
use firewheel::nodes::sampler::{RepeatMode, SamplerNode};
use firewheel_phonon::effects::spatializer::SpatializerNode;
use phonon::effects::binaural::BinauralEffectParameters;
use phonon::effects::direct::{DirectApplyFlags, DirectEffectParameters, TransmissionType};
use phonon::simulators::direct::DirectSoundPath;
use symphonium::SymphoniumLoader;

struct AudioSystem {
    cx: FirewheelContext,

    spatializer_node: Memo<SpatializerNode>,
    eq_node_id: NodeID,
}

impl AudioSystem {
    fn new() -> Self {
        let mut cx = FirewheelContext::new(Default::default());
        cx.start_stream(Default::default()).unwrap();

        let sample_rate = cx.stream_info().unwrap().sample_rate;

        let mut loader = SymphoniumLoader::new();
        let sample = firewheel::load_audio_file(
            &mut loader,
            "data/audio/pink_noise.ogg",
            Some(sample_rate),
            Default::default(),
        )
        .unwrap()
        .into_dyn_resource();

        let graph_out_node_id = cx.graph_out_node_id();

        let mut sampler_node = SamplerNode::default();
        sampler_node.set_sample(sample);
        sampler_node.repeat_mode = RepeatMode::RepeatEndlessly;
        sampler_node.start_or_restart();

        let spatializer_node = SpatializerNode::default();

        let sampler_node_id = cx.add_node(sampler_node.clone(), None);
        let spatializer_node_id = cx.add_node(spatializer_node.clone(), None);

        cx.connect(
            sampler_node_id,
            spatializer_node_id,
            &[(0, 0), (1, 0)],
            false,
        )
        .unwrap();
        cx.connect(
            spatializer_node_id,
            graph_out_node_id,
            &[(0, 0), (1, 1)],
            false,
        )
        .unwrap();

        Self {
            cx,
            spatializer_node: Memo::new(spatializer_node),
            eq_node_id: spatializer_node_id,
        }
    }

    fn update(&mut self) {
        if let Err(e) = self.cx.update() {
            println!("{:?}", &e);

            if let UpdateError::StreamStoppedUnexpectedly(_) = e {
                // The stream has stopped unexpectedly (i.e the user has
                // unplugged their headphones.)
                //
                // Typically you should start a new stream as soon as
                // possible to resume processing (event if it's a dummy
                // output device).
                //
                // In this example we just quit the application.
                panic!("Stream stopped unexpectedly!");
            }
        }
    }
}

fn main() {
    let mut audio_system = AudioSystem::new();
    let mut direct_params = DirectEffectParameters {
        direct_sound_path: DirectSoundPath::default(),
        flags: DirectApplyFlags::none(),
        transmission_type: TransmissionType::FrequencyDependent,
    };
    let mut binaural_params = BinauralEffectParameters::default();

    direct_params.flags.air_absorption = true;
    direct_params.flags.occlusion = true;
    direct_params.flags.transmission = true;

    eframe::run_simple_native(
        "Spatializer Effect (Firewheel)",
        eframe::NativeOptions::default(),
        move |ctx, _frame| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.add(egui::Checkbox::new(
                    &mut direct_params.flags.distance_attenuation,
                    "Apply distance attenuation",
                ));

                ui.add(
                    egui::Slider::new(
                        &mut direct_params.direct_sound_path.distance_attenuation,
                        0.0..=1.0,
                    )
                    .text("Distance Attenuation"),
                );

                ui.add(egui::Separator::default());
                ui.label("Air Absorption (AA) parameters:");

                ui.add(
                    egui::Slider::new(
                        &mut direct_params.direct_sound_path.air_absorption[0],
                        0.0..=1.0,
                    )
                    .text("AA Low"),
                );

                ui.add(
                    egui::Slider::new(
                        &mut direct_params.direct_sound_path.air_absorption[1],
                        0.0..=1.0,
                    )
                    .text("AA Mid"),
                );

                ui.add(
                    egui::Slider::new(
                        &mut direct_params.direct_sound_path.air_absorption[2],
                        0.0..=1.0,
                    )
                    .text("AA High"),
                );

                ui.add(egui::Separator::default());

                ui.add(
                    egui::Slider::new(&mut direct_params.direct_sound_path.occlusion, 0.0..=1.0)
                        .text("Occlusion factor"),
                );

                ui.add(
                    egui::Slider::new(
                        &mut direct_params.direct_sound_path.transmission[0],
                        0.0..=1.0,
                    )
                    .text("Transmission Low"),
                );

                ui.add(
                    egui::Slider::new(
                        &mut direct_params.direct_sound_path.transmission[1],
                        0.0..=1.0,
                    )
                    .text("Transmission Mid"),
                );

                ui.add(
                    egui::Slider::new(
                        &mut direct_params.direct_sound_path.transmission[2],
                        0.0..=1.0,
                    )
                    .text("Transmission High"),
                );

                ui.add(egui::Separator::default());

                ui.add(
                    egui::Slider::new(&mut binaural_params.direction.x, -1.0..=1.0)
                        .text("Direction X"),
                );

                ui.add(
                    egui::Slider::new(&mut binaural_params.direction.y, -1.0..=1.0)
                        .text("Direction Y"),
                );

                ui.add(
                    egui::Slider::new(&mut binaural_params.direction.z, -1.0..=1.0)
                        .text("Direction Z"),
                );
            });

            audio_system.spatializer_node.direct_effect_parameters = direct_params;
            audio_system.spatializer_node.binaural_effect_parameters = binaural_params;
            audio_system
                .spatializer_node
                .update_memo(&mut audio_system.cx.event_queue(audio_system.eq_node_id));
            audio_system.update();
        },
    )
    .unwrap()
}
