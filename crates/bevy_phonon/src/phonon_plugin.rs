use crate::phonon_mesh::instancing::StaticMeshes;
use crate::{AudioListener, phonon_mesh};
use bevy::prelude::*;
use bevy_seedling::prelude::{EffectsQuery, SampleEffects};
use bevy_seedling::sample::SamplePlayer;
use firewheel_phonon::effects::spatializer::SpatializerNode;
use firewheel_phonon::phonon;
use firewheel_phonon::phonon::models::air_absorption::DefaultAirAbsorptionModel;
use firewheel_phonon::phonon::models::distance_attenuation::DefaultDistanceAttenuationModel;
use firewheel_phonon::phonon::scene::coordinate_space::CoordinateSpace3f;
use firewheel_phonon::phonon::simulators::direct::{DirectSimulator, DirectSoundPath};

#[derive(Resource)]
pub(crate) struct SteamSimulation {
    pub(crate) simulator: DirectSimulator,
    pub(crate) scene: phonon::scene::Scene,
}

pub struct PhononPlugin {
    /// Sets the maximum number of occlusion samples, which is used when volumetric
    /// occlusion is enabled on a `PhononSource`.
    /// This only sets the max, the actual amount is set per source
    pub max_occlusion_samples: usize,
}

impl Default for PhononPlugin {
    fn default() -> Self {
        PhononPlugin {
            max_occlusion_samples: 128,
        }
    }
}

impl Plugin for PhononPlugin {
    fn build(&self, app: &mut App) {
        // This is the main scene to which all the geometry will be added
        let scene = phonon::scene::Scene::new();

        let simulator = DirectSimulator::new(self.max_occlusion_samples);

        app.insert_resource(SteamSimulation { simulator, scene })
            .insert_resource(StaticMeshes::default())
            //.register_type::<PhononSource>() todo
            .add_systems(
                Update,
                (
                    (
                        phonon_mesh::register_audio_meshes,
                        phonon_mesh::update_audio_mesh_transforms,
                    ),
                    update_steam_audio,
                )
                    .chain(),
            );
    }
}

fn update_steam_audio(
    mut sim_res: ResMut<SteamSimulation>,
    listener_query: Query<&GlobalTransform, With<AudioListener>>,
    mut audio_sources: Query<(&GlobalTransform, &mut SampleEffects), With<SamplePlayer>>,
    mut spatializer_nodes: Query<&mut SpatializerNode>,
) {
    // Commit changes to the sources, listener and scene.
    sim_res.scene.commit();

    let Ok(listener_transform) = listener_query.single() else {
        warn_once!("No audio listener was found");
        return;
    };

    let listener_position = CoordinateSpace3f::from_vectors(
        listener_transform.forward().into(),
        listener_transform.up().into(),
        listener_transform.translation(),
    );

    for (source_transform, effects) in audio_sources.iter_mut() {
        // todo remove unwrap
        let mut effect = spatializer_nodes.get_effect_mut(&effects).unwrap();
        let flags = effect.direct_effect_parameters.flags;
        let settings = effect.simulator_settings;

        let source_position = CoordinateSpace3f::from_vectors(
            source_transform.forward().into(),
            source_transform.up().into(),
            source_transform.translation(),
        );
        let direction = source_transform
            .reparented_to(listener_transform)
            .translation;

        let mut direct_sound_path = DirectSoundPath::default();

        sim_res.simulator.simulate(
            Some(&sim_res.scene),
            flags,
            &source_position,
            &listener_position,
            &DefaultDistanceAttenuationModel::default(),
            &DefaultAirAbsorptionModel::default(),
            settings.directivity,
            settings.occlusion_type,
            settings.occlusion_radius,
            settings.occlusion_samples,
            settings.num_transmission_rays,
            &mut direct_sound_path,
        );

        effect.direct_effect_parameters.direct_sound_path = direct_sound_path;
        // Note the change in coordinate systems here
        effect.binaural_effect_parameters.direction.x = direction.x;
        effect.binaural_effect_parameters.direction.y = -direction.z;
        effect.binaural_effect_parameters.direction.z = direction.y;
    }
}
