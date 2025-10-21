use crate::phonon_mesh;
use crate::phonon_mesh::instancing::StaticMeshes;
use bevy::prelude::*;
use bevy_fmod::prelude::AudioListener;
use bevy_fmod::prelude::AudioSource;
use libfmod::{Dsp, EventInstance};
use steamaudio::context::Context;
use steamaudio::fmod;
use steamaudio::geometry::Orientation;
use steamaudio::hrtf::Hrtf;
use steamaudio::simulation::{AirAbsorptionModel, DistanceAttenuationModel, Simulator, Source};

#[derive(Component)]
struct PhononSource {
    address: i32,
    source: Source,
}

#[derive(Component)]
pub struct PhononStaticMeshMarker;

//todo move or remove pub
#[derive(Resource)]
pub struct SteamSimulation {
    pub context: Context,
    pub hrtf: Hrtf,
    pub simulator: Simulator,
    pub scene: phonon_firewheel::phonon::scene::Scene,
}

pub struct PhononPlugin;

impl Plugin for PhononPlugin {
    fn build(&self, app: &mut App) {
        let sampling_rate = 48000; // Needs to be equal to FMOD sampling rate.
        let frame_size = 1024;
        let context = Context::new().unwrap();

        let hrtf = context.create_hrtf(sampling_rate, frame_size).unwrap();

        // This is the main scene to which all the geometry will be added later
        let scene = context.create_scene().unwrap();
        scene.commit();

        // todo! simulationsettings are pretty much hardcoded right now
        // simulation_settings.max_num_occlusion_samples = 8; // This only sets the max, the actual amount is set per source
        let mut simulator = context.create_simulator(sampling_rate, frame_size).unwrap();
        simulator.set_scene(&scene);
        simulator.set_reflections(4096, 16, 2.0, 1, 1.0);

        fmod::init_fmod(&context);
        fmod::set_hrtf(&hrtf);

        let settings = fmod::fmod_create_settings(sampling_rate, frame_size);
        fmod::set_simulation_settings(settings);

        app.insert_resource(SteamSimulation {
            simulator,
            context,
            hrtf,
            scene,
        })
        .insert_resource(StaticMeshes::default())
        .add_systems(
            Update,
            (
                (
                    register_phonon_sources,
                    phonon_mesh::register_audio_meshes,
                    phonon_mesh::update_audio_mesh_transforms,
                    update_steam_audio_listener,
                    update_steam_audio_source,
                ),
                update_steam_audio,
            )
                .chain(),
        );
    }
}

fn update_steam_audio_listener(
    mut sim_res: ResMut<SteamSimulation>,
    listener_query: Query<&GlobalTransform, With<AudioListener>>,
) {
    let listener_transform = listener_query.get_single().unwrap();
    let (_rotation, rotation, translation) = listener_transform.to_scale_rotation_translation();

    sim_res.simulator.set_listener(Orientation {
        translation,
        rotation,
    });
}

fn update_steam_audio_source(mut source_query: Query<(&GlobalTransform, &mut PhononSource)>) {
    for (source_transform, mut phonon_source) in source_query.iter_mut() {
        let (_rotation, rotation, translation) = source_transform.to_scale_rotation_translation();

        phonon_source.source.set_source(Orientation {
            translation,
            rotation,
        });
    }
}

fn update_steam_audio(sim_res: ResMut<SteamSimulation>) {
    // Commit changes to the sources, listener and scene.
    sim_res.simulator.commit();

    sim_res.simulator.run_direct();
    sim_res.simulator.run_reflections(); //todo make optional

    // The Steam Audio FMOD plugin will periodically collect the simulation outputs
    // as long as the plugin has handles to the Steam Audio sources.
    // See function `register_phonon_sources`.
}

/// Currently all bevy_fmod audio sources will be converted to Steam Audio sources.
fn register_phonon_sources(
    mut audio_sources: Query<(Entity, &AudioSource), Without<PhononSource>>,
    mut commands: Commands,
    sim_res: Res<SteamSimulation>,
) {
    for (audio_entity, audio_source_fmod) in audio_sources.iter_mut() {
        if let Some(phonon_dsp) = get_phonon_spatializer(audio_source_fmod.event_instance) {
            let mut source = sim_res.simulator.create_source(true).unwrap();
            source.set_distance_attenuation(DistanceAttenuationModel::Default);
            source.set_air_absorption(AirAbsorptionModel::Default);
            source.set_occlusion();
            source.set_transmission(1);
            source.set_reflections();
            source.set_active(true);

            let source_address = fmod::add_source(&source);
            let simulation_outputs_parameter_index = 33; //todo explain where this number comes from

            // By setting this field the Steam Audio FMOD plugin can retrieve the
            // simulation results like occlusion and reflection.
            phonon_dsp
                .set_parameter_int(simulation_outputs_parameter_index, source_address)
                .unwrap();

            commands.entity(audio_entity).insert(PhononSource {
                address: source_address,
                source,
            });
        }
    }
}
