use bevy::prelude::*;

use crate::scenes::SceneSelection;

struct

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
        DespawnOnExit(SceneSelection::Intro),
    ));
    // Add a fog volume.
    // commands.spawn((
    //     FogVolume::default(),
    //     Transform::from_scale(Vec3::splat(35.0)),
    // ));
}

// todo move this, adjust the scale
/// Moves everything up so that the atmosphere looks it bit more atmospheric
fn into_the_sky(mut tfs: Query<&mut Transform>) {
    for mut tf in tfs.iter_mut() {
        tf.translation.y = tf.translation.y + 5000.0;
    }
}
