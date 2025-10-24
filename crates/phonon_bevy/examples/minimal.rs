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

fn startup(
    assets: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Let's spawn a looping sample.
    commands.spawn((
        SamplePlayer::new(assets.load("dpren_very-lush-and-swag-loop.ogg")).looping(),
        sample_effects![SpatializerNode::default()],
        Transform::default(),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
    ));
}

// todo test with multiple sample players

fn update(
    mut custom_node: Query<&mut Transform, With<SpatializerNode>>,
    time: Res<Time>,
) -> Result {
    let mut tf = custom_node.single_mut()?;

    let period = 5.0;
    let todo = (time.elapsed_secs() * core::f32::consts::TAU / period).sin();

    tf.translation.x = todo;

    Ok(())
}
