use bevy::{
    light::{CascadeShadowConfigBuilder, light_consts::lux},
    prelude::*,
};

/// set up a simple 3D scene
pub(crate) fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    // todo camera position
    // Sun
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: lux::RAW_SUNLIGHT,
            ..default()
        },
        Transform::from_xyz(1.0, 0.2, 0.3).looking_at(Vec3::ZERO, Vec3::Y),
        CascadeShadowConfigBuilder::default().build(),
        //VolumetricLight,
    ));
    // Add a fog volume.
    // commands.spawn((
    //     FogVolume::default(),
    //     Transform::from_scale(Vec3::splat(35.0)),
    // ));
}
