// todo: List controls

use bevy::prelude::*;
use bevy_flycam::prelude::*;
use std::sync::Arc;

use bevy_kira_components::kira::sound::Region;
use bevy_kira_components::kira::track::TrackBuilder;

use bevy_kira_components::prelude::*;
use bevy_kira_components::tracks::TrackHandle;
use bevy_kira_components::AudioPlugin;
use phonon::air_absorption::DefaultAirAbsorptionModel;
use phonon::coordinate_space::CoordinateSpace3f;
use phonon::direct_effect::{DirectApplyFlags, DirectEffectParameters, TransmissionType};
use phonon::direct_simulator::{DirectSimulator, DirectSoundPath, OcclusionType};
use phonon::directivity::Directivity;
use phonon::distance_attenuation::DefaultDistanceAttenuationModel;
use phonon::panning_effect::PanningEffectParameters;
use phonon::static_mesh::StaticMesh;
use phonon_kira::direct_effect::builder::DirectEffectBuilder;
use phonon_kira::direct_effect::handle::DirectEffectHandle;

#[derive(Component)]
struct DirectTrack;

#[derive(Component)]
struct SourceMarker;

#[derive(Resource)]
struct Phonon {
    simulator: DirectSimulator,
    scene: phonon::scene::Scene,
}

fn main() {
    let max_occlusion_samples = 100;

    let mesh =
        phonon::mesh::Mesh::new_from_parry(parry3d::shape::Cuboid::new([1.0, 1.0, 1.0].into()));
    let static_mesh = Arc::new(StaticMesh::new_from_mesh(mesh));

    let mut scene = phonon::scene::Scene::new();
    scene.add_static_mesh(static_mesh);
    scene.commit();

    App::new()
        .insert_resource(Phonon {
            simulator: DirectSimulator::new(max_occlusion_samples),
            scene,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(AudioPlugin)
        .add_systems(Startup, (setup, setup_track))
        .add_systems(Update, (init_camera, init_sound, update_direct_effect))
        .run();
}

fn init_camera(mut commands: Commands, camera_query: Query<Entity, Added<Camera>>) {
    if let Ok(camera) = camera_query.get_single() {
        commands.entity(camera).insert(AudioListener);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
            .with_translation(Vec3::new(0.0, -1.0, 0.0)),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(2.0, 2.0, 2.0)),
        material: materials.add(Color::rgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn setup_track(mut commands: Commands) {
    let direct_params = DirectEffectParameters {
        direct_sound_path: DirectSoundPath::default(),
        flags: DirectApplyFlags::DistanceAttenuation | DirectApplyFlags::Occlusion,
        transmission_type: TransmissionType::FrequencyIndependent,
    };

    let mut track = TrackBuilder::new();
    let panning = track.add_effect(DirectEffectBuilder {
        parameters: direct_params,
        panning_params: Default::default(),
    });
    // Spawn track entity
    commands.spawn((Track(track), EffectHandle(panning), DirectTrack));
}

fn init_sound(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    track_query: Query<Entity, Added<TrackHandle>>,
) {
    if let Ok(track_ent) = track_query.get_single() {
        // todo: consider moving the data folder
        let audio_file = asset_server.load::<AudioFile>("../../../data/audio/pink_noise.ogg");

        // Audio emitter
        commands
            .spawn((
                SourceMarker,
                InheritedVisibility::VISIBLE,
                TransformBundle {
                    local: Transform::from_xyz(0., 1., -6.0),
                    ..default()
                },
            ))
            .with_children(|children| {
                children.spawn((
                    //SpatialEmitter::default(), // todo this breaks the track routing
                    bevy_kira_components::prelude::AudioBundle {
                        source: audio_file,
                        settings: AudioFileSettings {
                            loop_region: Some(Region::from(..)),
                            ..default()
                        },
                        output: OutputDestination::SpecificTrack(track_ent),
                        ..default()
                    },
                    PbrBundle {
                        mesh: meshes.add(Sphere::new(0.1).mesh()),
                        material: materials.add(StandardMaterial {
                            base_color: Color::WHITE,
                            emissive: Color::GREEN,
                            ..default()
                        }),
                        transform: Transform::from_xyz(0., 0., 0.0),
                        ..default()
                    },
                ));
            });
    }
}

fn update_direct_effect(
    cam_query: Query<&GlobalTransform, With<Camera>>,
    audio_source_query: Query<&GlobalTransform, With<SourceMarker>>,
    mut effect_query: Query<&mut EffectHandle<DirectEffectHandle>>,
    phonon_res: Res<Phonon>,
) {
    let cam_transform = cam_query.get_single().unwrap();
    let source_transform = audio_source_query.get_single();

    if source_transform.is_err() {
        return;
    }

    let source_transform = source_transform.unwrap();

    for mut effect in &mut effect_query {
        let num_samples_source = 100; // must be less than `max_occlusion_samples`

        let flags = DirectApplyFlags::DistanceAttenuation
            | DirectApplyFlags::AirAbsorption
            | DirectApplyFlags::Occlusion
            | DirectApplyFlags::Transmission;

        let source_position = CoordinateSpace3f::from_vectors(
            cam_transform.forward(),
            cam_transform.up(),
            cam_transform.translation(),
        );
        let listener_position = CoordinateSpace3f::from_vectors(
            source_transform.forward(),
            source_transform.up(),
            source_transform.translation(),
        );

        let mut direct_sound_path = DirectSoundPath::default();

        phonon_res.simulator.simulate(
            &phonon_res.scene,
            flags,
            source_position,
            listener_position,
            &DefaultDistanceAttenuationModel::default(),
            &DefaultAirAbsorptionModel::default(),
            Directivity::default(),
            OcclusionType::Raycast,
            1.0,
            num_samples_source,
            1,
            &mut direct_sound_path,
        );

        effect
            .0
            .set_parameters(DirectEffectParameters {
                direct_sound_path,
                flags,
                transmission_type: TransmissionType::FrequencyDependent,
            })
            .unwrap();

        // Todo: The following probably doesn't need to use the Steam Audio coordinate utilities
        let coordinates = CoordinateSpace3f::from_vectors(
            cam_transform.forward(),
            cam_transform.up(),
            cam_transform.translation(),
        );
        let point = source_transform.translation();
        let world_space_direction = (point - coordinates.origin).normalize_or_zero();
        let local_space_direction = coordinates.direction_to_local(world_space_direction);

        effect
            .0
            .set_panning(PanningEffectParameters {
                direction: local_space_direction,
            })
            .unwrap();
    }
}
