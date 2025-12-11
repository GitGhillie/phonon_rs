use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::*;
use bevy_phonon::effects::spatializer::SpatializerNode;
use bevy_seedling::{sample::SamplePlayer, sample_effects};

use crate::{
    DemoAssets,
    scenes::{DemoScene, SceneSelection, TextAssets},
};

pub(crate) struct DistanceEffectsDemo;

impl DemoScene for DistanceEffectsDemo {
    fn setup_systems(&self, app: &mut App, schedule: impl ScheduleLabel) {
        app.add_systems(schedule, (setup, setup_ui));
    }

    fn update_systems(&self, app: &mut App, schedule: impl ScheduleLabel) {
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
}

fn move_cubes(mut cubes: Query<&mut Transform, With<MoveMarker>>, time: Res<Time>) {
    let period = 5.0;
    let position = (time.elapsed_secs() * core::f32::consts::TAU / period).sin() * 10.0;

    for mut cube_tf in cubes.iter_mut() {
        cube_tf.translation.x = position;
    }
}

fn setup_ui(mut commands: Commands) {
    let text = String::from_utf8(
        TextAssets::get("distance_effects.md")
            .unwrap()
            .data
            .to_vec(),
    );
    commands.spawn((
        DespawnOnExit(SceneSelection::DistanceAttenuation),
        Text::from(text.unwrap()),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(5),
            left: px(15),
            ..default()
        },
    ));
}
