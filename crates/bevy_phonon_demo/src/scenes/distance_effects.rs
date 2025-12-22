use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::*;
use bevy_phonon::effects::spatializer::SpatializerNode;
use bevy_seedling::{
    prelude::{EffectsQuery, SampleEffects},
    sample::SamplePlayer,
    sample_effects,
};

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
        app.add_systems(
            schedule,
            (move_cubes, controls, update_ui).run_if(in_state(SceneSelection::DistanceAttenuation)),
        );
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
    info!("Setting up scene");
    commands.spawn((
        Name::from("Cube Translating"),
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

fn controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player: Single<&SampleEffects, With<SamplePlayer>>,
    mut effects: Query<&mut SpatializerNode>,
) -> Result {
    let mut effect = effects.get_effect_mut(&player)?;

    if keyboard_input.just_pressed(KeyCode::Digit1) {
        info!("toggle dist");
        effect.direct_effect_parameters.flags.distance_attenuation ^= true;
    }
    if keyboard_input.just_pressed(KeyCode::Digit2) {
        effect.direct_effect_parameters.flags.air_absorption ^= true;
    }
    if keyboard_input.just_pressed(KeyCode::Digit3) {
        effect.direct_effect_parameters.flags.delay ^= true;
    }

    Ok(())
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
        text_shadow_component(),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(5),
            left: px(15),
            ..default()
        },
    ));

    commands.spawn((
        DespawnOnExit(SceneSelection::DistanceAttenuation),
        Text::from("A"),
        text_shadow_component(),
        Name::from("StatusText"),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(5),
            right: px(15),
            ..default()
        },
    ));
}

fn update_ui(
    player: Single<&SampleEffects, With<SamplePlayer>>,
    effects: Query<&SpatializerNode>,
    mut text: Query<&mut TextColor>,
) -> Result {
    let effect = effects.get_effect(&player)?;
    let distance_attenuation = effect.direct_effect_parameters.flags.distance_attenuation;

    Ok(())
}

fn text_shadow_component() -> TextShadow {
    TextShadow {
        offset: Vec2 { x: 2.0, y: 2.0 },
        ..Default::default()
    }
}
