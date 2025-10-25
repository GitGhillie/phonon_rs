//! Minimal example spawning a sound with spatializer effect.
//! The effect applies the Direct Simulation (distance attenuation, air absorption, etc.)
//! and spatializes it with a HRTF.

use bevy::prelude::*;
use bevy_phonon::{AudioListener, effects::spatializer::SpatializerNode, prelude::PhononPlugin};
use bevy_seedling::{SeedlingPlugin, node::RegisterNode, sample::SamplePlayer, sample_effects};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SeedlingPlugin::default())
        .add_plugins(PhononPlugin::default())
        .register_node::<SpatializerNode>()
        .add_systems(Startup, startup)
        .add_systems(Update, update)
        .run();
}

fn startup(
    assets: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
        AudioListener,
        Camera3d::default(),
        Transform::from_xyz(6.0, 3.5, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    // Looping sample player attached to a cube.
    commands.spawn((
        SamplePlayer::new(assets.load("dpren_very-lush-and-swag-loop.ogg")).looping(),
        sample_effects![SpatializerNode::default()],
        Transform::default(),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
    ));
}

fn update(mut custom_node: Query<&mut Transform, With<SamplePlayer>>, time: Res<Time>) {
    let period = 5.0;
    let position = (time.elapsed_secs() * core::f32::consts::TAU / period).sin() * 10.0;

    for mut tf in custom_node.iter_mut() {
        tf.translation = Vec3::splat(position);
    }
}
