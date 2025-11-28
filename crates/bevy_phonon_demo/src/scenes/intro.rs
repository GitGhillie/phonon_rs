use bevy::prelude::*;
use bevy_phonon::effects::spatializer::SpatializerNode;

use crate::{
    DemoAssets,
    scenes::{DemoScene, SceneSelection, TextAssets},
};
use bevy_seedling::{sample::SamplePlayer, sample_effects};

pub(crate) struct IntroDemo;

impl DemoScene for IntroDemo {
    fn setup_systems(&self, app: &mut App, schedule: impl bevy::ecs::schedule::ScheduleLabel) {
        app.add_systems(schedule, (setup_scene, setup_ui));
    }
}

/// set up a simple 3D scene
fn setup_scene(
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

fn setup_ui(mut commands: Commands) {
    let text = String::from_utf8(TextAssets::get("intro.md").unwrap().data.to_vec());
    commands.spawn((
        DespawnOnExit(SceneSelection::Intro),
        Text::from(text.unwrap()),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(5),
            left: px(15),
            ..default()
        },
    ));
}

// todo move this, adjust the scale
/// Moves everything up so that the atmosphere looks it bit more atmospheric
fn into_the_sky(mut tfs: Query<&mut Transform>) {
    for mut tf in tfs.iter_mut() {
        tf.translation.y = tf.translation.y + 5000.0;
    }
}
