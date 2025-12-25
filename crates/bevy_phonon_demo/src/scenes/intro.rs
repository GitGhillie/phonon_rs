use bevy::prelude::*;
use bevy_phonon::effects::spatializer::SpatializerNode;

use crate::{
    DemoAssets,
    scenes::{DemoScene, SceneSelection, TextAssets, text_shadow_component},
};
use bevy_seedling::{sample::SamplePlayer, sample_effects};

pub(crate) struct IntroDemo;

impl DemoScene for IntroDemo {
    fn setup_systems(&self, app: &mut App, schedule: impl bevy::ecs::schedule::ScheduleLabel) {
        app.add_systems(schedule, (setup_scene, setup_ui));
    }
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    demo_assets: Res<DemoAssets>,
) {
    info!("Setting up scene");
    commands.spawn((
        Name::from("Cube Intro"),
        SamplePlayer::new(demo_assets.audio_sample.clone()).looping(),
        sample_effects![SpatializerNode::default()],
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
        DespawnOnExit(SceneSelection::Intro),
    ));
}

fn setup_ui(mut commands: Commands) {
    let text = String::from_utf8(TextAssets::get("intro.md").unwrap().data.to_vec());
    commands.spawn((
        DespawnOnExit(SceneSelection::Intro),
        Text::from(text.unwrap()),
        text_shadow_component(),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(5),
            left: px(15),
            ..default()
        },
    ));
}
