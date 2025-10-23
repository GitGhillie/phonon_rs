use crate::phonon_mesh::instancing::StaticMeshes;
use crate::{AudioListener, phonon_mesh};
use bevy::prelude::*;
use phonon_firewheel::effects::spatializer::SpatializerNode;
use phonon_firewheel::phonon;
use phonon_firewheel::phonon::effects::direct::DirectApplyFlags;
use phonon_firewheel::phonon::models::air_absorption::DefaultAirAbsorptionModel;
use phonon_firewheel::phonon::models::directivity::Directivity;
use phonon_firewheel::phonon::models::distance_attenuation::DefaultDistanceAttenuationModel;
use phonon_firewheel::phonon::scene::coordinate_space::CoordinateSpace3f;
use phonon_firewheel::phonon::simulators::direct::{
    DirectSimulator, DirectSoundPath, OcclusionType,
};
use std::os::raw::c_void;

#[derive(Component)]
pub struct PhononSource {
    pub distance_attenuation: bool,
    pub air_absorption: bool,
    pub occlusion: bool,
    pub occlusion_type: OcclusionType,
    /// Size of the audio source when `OcclusionType` is set to `Volumetric`.
    pub occlusion_radius: f32,
    /// Number of occlusion samples to take when volumetric occlusion is enabled.
    /// Limited by `max_occlusion_samples` of the `DirectSimulator`.
    pub occlusion_samples: usize,
    // todo document what transmission is and what is needed to make it work (materials)
    pub transmission: bool,
    pub directivity: bool,
    pub hrtf_enable: bool,
}

impl Default for PhononSource {
    fn default() -> Self {
        PhononSource {
            distance_attenuation: true,
            air_absorption: true,
            occlusion: true,
            occlusion_type: OcclusionType::Volumetric,
            occlusion_radius: 1.0,
            occlusion_samples: 64,
            transmission: true,
            directivity: true,
            hrtf_enable: true,
        }
    }
}

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
            max_occlusion_samples: 512,
        }
    }
}

impl Plugin for PhononPlugin {
    fn build(&self, app: &mut App) {
        // This is the main scene to which all the geometry will be added later
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
                    phonon_source_changed,
                )
                    .chain(),
            );
    }
}

fn update_steam_audio(
    mut sim_res: ResMut<SteamSimulation>,
    listener_query: Query<&GlobalTransform, With<AudioListener>>,
    audio_sources: Query<(&GlobalTransform, &SpatializerNode)>,
) {
    // Commit changes to the sources, listener and scene.
    sim_res.scene.commit();

    let listener_transform = listener_query.get_single().unwrap();

    let listener_position = CoordinateSpace3f::from_vectors(
        listener_transform.forward().into(),
        listener_transform.up().into(),
        listener_transform.translation(),
    );

    for (source_transform, effect) in audio_sources.iter() {
        let mut flags = DirectApplyFlags::none();
        flags.distance_attenuation = settings.distance_attenuation;

        flags.air_absorption = settings.air_absorption;
        flags.occlusion = settings.occlusion;
        flags.transmission = settings.transmission;
        flags.directivity = settings.directivity;

        let source_position = CoordinateSpace3f::from_vectors(
            source_transform.forward().into(),
            source_transform.up().into(),
            source_transform.translation(),
        );

        let mut direct_sound_path = DirectSoundPath::default();

        let directivity = match settings.directivity {
            true => {
                let valuestrlen = 0;
                let (directivity_power, _) = spatializer
                    .get_parameter_float(Params::DirectivityDipolePower as i32, valuestrlen)
                    .unwrap();
                let (directivity_weight, _) = spatializer
                    .get_parameter_float(Params::DirectivityDipoleWeight as i32, valuestrlen)
                    .unwrap();
                Directivity {
                    dipole_weight: directivity_weight,
                    dipole_power: directivity_power,
                }
            }
            false => Directivity::default(),
        };

        sim_res.simulator.simulate(
            Some(&sim_res.scene),
            flags,
            &source_position,
            &listener_position,
            &DefaultDistanceAttenuationModel::default(),
            &DefaultAirAbsorptionModel::default(),
            directivity,
            settings.occlusion_type,
            settings.occlusion_radius,
            settings.occlusion_samples,
            1,
            &mut direct_sound_path,
        );

        let sound_path_ptr = &mut direct_sound_path as *mut _ as *mut c_void;
        let sound_path_size = size_of::<DirectSoundPath>();

        spatializer
            .set_parameter_data(
                Params::DirectSoundPath as i32,
                sound_path_ptr,
                sound_path_size as u32,
            )
            .unwrap();
    }
}
