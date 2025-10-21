//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;
use bevy_seedling::{
    SeedlingPlugin,
    node::RegisterNode,
    prelude::{EffectsQuery, SampleEffects},
    sample::SamplePlayer,
    sample_effects,
};
use phonon_bevy::effects::spatializer::SpatializerNode;
use phonon_firewheel::phonon::effects::{
    binaural::BinauralEffectParameters,
    direct::{DirectApplyFlags, DirectEffectParameters},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SeedlingPlugin::default())
        .register_node::<SpatializerNode>()
        .add_systems(Startup, (setup, startup))
        .add_systems(Update, update)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn startup(server: Res<AssetServer>, mut commands: Commands) {
    // todo implement default on spatializer node
    let mut direct_effect_parameters = DirectEffectParameters::default();
    let binaural_effect_parameters = BinauralEffectParameters::default();

    // todo
    direct_effect_parameters.flags = DirectApplyFlags::none();
    direct_effect_parameters.flags.air_absorption = true;
    direct_effect_parameters.flags.distance_attenuation = true;
    direct_effect_parameters.flags.occlusion = true;

    // Let's spawn a looping sample.
    commands.spawn((
        SamplePlayer::new(server.load("dpren_very-lush-and-swag-loop.ogg")).looping(),
        sample_effects![SpatializerNode {
            direct_effect_parameters,
            binaural_effect_parameters
        }],
    ));
}

// Here we'll see how simply mutating the parameters
// will be automatically synchronized with the audio processor.
fn update(
    player: Single<&SampleEffects, With<SamplePlayer>>,
    mut custom_node: Query<&mut SpatializerNode>,
    time: Res<Time>,
    mut angle: Local<f32>,
) -> Result {
    let mut custom_node = custom_node.get_effect_mut(&player)?;

    custom_node.binaural_effect_parameters.direction = Vec3 {
        x: angle.cos(),
        y: angle.sin(),
        z: 0.0,
    };

    let period = 5.0;
    *angle += time.delta().as_secs_f32() * core::f32::consts::TAU / period;

    Ok(())
}
