use bevy::prelude::*;
use bevy_phonon::effects::spatializer::SpatializerNode;
use bevy_seedling::{sample::SamplePlayer, sample_effects};

use crate::{
    DemoAssets,
    scenes::{DemoScene, SceneSelection},
};

pub(crate) struct DistanceEffectsDemo;

impl DemoScene for DistanceEffectsDemo {
    fn setup_systems(&self, app: &mut App, schedule: impl bevy::ecs::schedule::ScheduleLabel) {
        app.add_systems(schedule, setup);
    }

    fn update_systems(&self, app: &mut App, schedule: impl bevy::ecs::schedule::ScheduleLabel) {
        app.add_systems(schedule, move_cubes);
    }
}

#[derive(Component)]
struct MoveMarker;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    demo_assets: Res<DemoAssets>,
) {
    // cube
    commands.spawn((
        SamplePlayer::new(demo_assets.audio_sample.clone()).looping(),
        sample_effects![SpatializerNode::default()],
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(233, 1, 1))),
        Transform::from_xyz(0.0, 1.5, 0.0),
        MoveMarker,
        DespawnOnExit(SceneSelection::DistanceAttenuation),
    ));
    // todo camera position

    // Add a fog volume.
    // commands.spawn((
    //     FogVolume::default(),
    //     Transform::from_scale(Vec3::splat(35.0)),
    // ));
}

fn move_cubes(mut cubes: Query<&mut Transform, With<MoveMarker>>, time: Res<Time>) {
    let period = 5.0;
    let position = (time.elapsed_secs() * core::f32::consts::TAU / period).sin() * 10.0;

    for mut cube_tf in cubes.iter_mut() {
        cube_tf.translation.x = position;
    }
}
