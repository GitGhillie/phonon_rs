// todo: List controls

use bevy::prelude::*;
use bevy_flycam::prelude::*;

use bevy_kira_components::kira::sound::Region;
use bevy_kira_components::kira::track::TrackBuilder;
use bevy_kira_components::kira::tween::Tween;
use bevy_kira_components::prelude::*;
use bevy_kira_components::AudioPlugin;
use phonon::direct_effect::{DirectApplyFlags, DirectEffectParameters, TransmissionType};
use phonon::direct_simulator::DirectSoundPath;
use phonon_kira::direct_effect::builder::DirectEffectBuilder;
use phonon_kira::direct_effect::handle::DirectEffectHandle;

#[derive(Component)]
struct DirectTrack;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(AudioPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (init_camera, update_direct_effect))
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
    asset_server: Res<AssetServer>,
) {
    let mut direct_params = DirectEffectParameters {
        direct_sound_path: DirectSoundPath::default(),
        flags: DirectApplyFlags::DistanceAttenuation | DirectApplyFlags::Occlusion,
        transmission_type: TransmissionType::FrequencyIndependent,
    };

    // todo: consider moving the data folder
    let source = asset_server.load("../../../data/audio/pink_noise.ogg");

    let mut track_builder = TrackBuilder::new();

    let effect_handle = track_builder.add_effect(DirectEffectBuilder {
        parameters: direct_params,
    });

    let track_entity = commands
        .spawn((
            Track(track_builder),
            EffectHandle(effect_handle),
            DirectTrack,
        ))
        .id();

    // Audio emitter
    commands
        .spawn((
            InheritedVisibility::VISIBLE,
            TransformBundle {
                local: Transform::from_xyz(0., 1., -6.0),
                ..default()
            },
        ))
        .with_children(|children| {
            children.spawn((
                SpatialEmitter::default(),
                AudioFileBundle {
                    source,
                    settings: AudioFileSettings {
                        loop_region: Some(Region::from(..)),
                        ..default()
                    },
                    output: OutputDestination::SpecificTrack(track_entity),
                    ..default()
                },
                PbrBundle {
                    mesh: meshes.add(Sphere::new(0.1).mesh()),
                    material: materials.add(StandardMaterial {
                        base_color: Color::WHITE,
                        emissive: Color::GREEN,
                        ..default()
                    }),
                    transform: Transform::from_xyz(0., 0., 2.5),
                    ..default()
                },
            ));
        });

    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::rgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
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

// todo: Do we need this DirectTrack filter?
fn update_direct_effect(
    mut effect_query: Query<&mut EffectHandle<DirectEffectHandle>, With<DirectTrack>>,
) {
    for mut effect in &mut effect_query {
        // todo: Get flags and transmission type instead of changing them.
        effect
            .0
            .set_parameters(DirectEffectParameters {
                direct_sound_path: DirectSoundPath {
                    distance_attenuation: 0.3,
                    air_absorption: [1.0, 1.0, 1.0],
                    delay: 0.0,
                    occlusion: 0.3,
                    transmission: [1.0, 1.0, 1.0],
                    directivity: 0.0,
                },
                flags: DirectApplyFlags::DistanceAttenuation | DirectApplyFlags::Occlusion,
                transmission_type: TransmissionType::FrequencyIndependent,
            })
            .unwrap();
    }
}
