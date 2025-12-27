use bevy::{color::palettes::css::ORANGE_RED, prelude::*};
use bevy_phonon::effects::spatializer::SpatializerNode;

use crate::{
    DemoAssets,
    scenes::{DemoScene, SceneSelection, TextAssets, text_shadow_component},
};
use bevy_seedling::{
    prelude::{EffectsQuery, SampleEffects},
    sample::SamplePlayer,
    sample_effects,
};

pub(crate) struct DirectivityDemo;

impl DemoScene for DirectivityDemo {
    fn setup_systems(&self, app: &mut App, schedule: impl bevy::ecs::schedule::ScheduleLabel) {
        app.add_systems(schedule, (setup_scene, setup_ui));
    }

    fn update_systems(&self, app: &mut App, schedule: impl bevy::ecs::schedule::ScheduleLabel) {
        app.add_systems(
            schedule,
            (rotate_cube, controls).run_if(in_state(SceneSelection::Directivity)),
        );
    }
}

// todo: Visualize the directivity pattern

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    demo_assets: Res<DemoAssets>,
) {
    info!("Setting up scene");
    commands.spawn((
        Name::from("Cube Directivity"),
        SamplePlayer::new(demo_assets.audio_sample.clone()).looping(),
        sample_effects![SpatializerNode::default()],
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(255, 144, 124))),
        Transform::from_xyz(0.0, 1.5, 0.0),
        DespawnOnExit(SceneSelection::Directivity),
    ));
}

fn setup_ui(mut commands: Commands) {
    let text = String::from_utf8(TextAssets::get("intro.md").unwrap().data.to_vec());
    commands.spawn((
        DespawnOnExit(SceneSelection::Directivity),
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

fn rotate_cube(
    mut cube_transforms: Query<&mut Transform, With<SamplePlayer>>,
    time: Res<Time<Virtual>>,
    mut gizmos: Gizmos,
) {
    for mut cube_transform in cube_transforms.iter_mut() {
        cube_transform.rotate_axis(Dir3::Y, time.delta_secs() * 1.0);
        let arrow_start = cube_transform.translation;
        let arrow_end = cube_transform.translation + *cube_transform.forward();
        gizmos.arrow(arrow_start, arrow_end, ORANGE_RED);
    }
}

fn controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player: Single<&SampleEffects, With<SamplePlayer>>,
    mut effects: Query<&mut SpatializerNode>,
) -> Result {
    let mut effect = effects.get_effect_mut(&player)?;

    if keyboard_input.just_pressed(KeyCode::Digit1) {
        effect.direct_effect_parameters.flags.directivity ^= true;
    }
    if keyboard_input.just_pressed(KeyCode::Digit2) {
        effect.simulator_settings.directivity.dipole_power = 1.0;
    }
    if keyboard_input.just_pressed(KeyCode::Digit3) {
        effect.simulator_settings.directivity.dipole_power = 2.0;
    }
    if keyboard_input.just_pressed(KeyCode::Digit4) {
        effect.simulator_settings.directivity.dipole_weight = 0.0;
    }
    if keyboard_input.just_pressed(KeyCode::Digit5) {
        effect.simulator_settings.directivity.dipole_weight = 1.0;
    }

    Ok(())
}
