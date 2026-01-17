use std::f32::consts::PI;

use bevy::{color::palettes::css::RED, prelude::*};
use bevy_phonon::effects::spatializer::SpatializerNode;

use crate::{
    DemoAssets,
    scenes::{DemoScene, SceneSelection, text_shadow_component},
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
            (rotate_cube, controls, update_ui, visualize_directivity)
                .run_if(in_state(SceneSelection::Directivity)),
        );
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
        Name::from("Cube Directivity"),
        SamplePlayer::new(demo_assets.audio_sample.clone()).looping(),
        sample_effects![SpatializerNode::default()],
        Mesh3d(meshes.add(Cuboid::from_length(0.25))),
        MeshMaterial3d(materials.add(Color::srgb_u8(255, 144, 124))),
        Transform::from_xyz(0.0, 1.5, 0.0),
        DespawnOnExit(SceneSelection::Directivity),
    ));
}

fn setup_ui(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(SceneSelection::Directivity),
        Text::from("StatusText"),
        text_shadow_component(),
        Name::from("StatusText"),
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
) {
    for mut cube_transform in cube_transforms.iter_mut() {
        cube_transform.rotate_axis(Dir3::Y, time.delta_secs() * 1.0);
    }
}

fn controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player: Single<&SampleEffects, With<SamplePlayer>>,
    mut effects: Query<&mut SpatializerNode>,
    time: Res<Time<Real>>,
) -> Result {
    let mut effect = effects.get_effect_mut(&player)?;
    let sensitivity_weight = 0.5;
    let sensitivity_power = 4.0 * sensitivity_weight;

    let mut weight = effect.simulator_settings.directivity.dipole_weight;
    let mut power = effect.simulator_settings.directivity.dipole_power;

    if keyboard_input.just_pressed(KeyCode::Digit1) {
        effect.direct_effect_parameters.flags.directivity ^= true;
    }
    if keyboard_input.pressed(KeyCode::Digit2) {
        weight -= sensitivity_weight * time.delta_secs();
    }
    if keyboard_input.pressed(KeyCode::Digit3) {
        weight += sensitivity_weight * time.delta_secs();
    }
    if keyboard_input.pressed(KeyCode::Digit4) {
        power -= sensitivity_power * time.delta_secs();
    }
    if keyboard_input.pressed(KeyCode::Digit5) {
        power += sensitivity_power * time.delta_secs();
    }

    effect.simulator_settings.directivity.dipole_power = power.clamp(1.0, 4.0);
    effect.simulator_settings.directivity.dipole_weight = weight.clamp(0.0, 1.0);

    Ok(())
}

fn update_ui(
    player: Single<&SampleEffects, With<SamplePlayer>>,
    effects: Query<&SpatializerNode>,
    mut text: Query<&mut Text, With<Name>>,
) -> Result {
    let effect = effects.get_effect(&player)?;
    let directivity = effect.direct_effect_parameters.flags.directivity;
    let power = effect.simulator_settings.directivity.dipole_power;
    let weight = effect.simulator_settings.directivity.dipole_weight;

    if let Ok(mut text) = text.single_mut() {
        let strings = [
            "Sound sources can emit sound with different intensities in different directions."
                .to_string(),
            "Press the following keys to affect the directivity:".to_string(),
            format!("[1] - Directivity enabled: {directivity}"),
            format!("[2/3] - Dipole weight: {weight:.3}"),
            format!("[4/5] - Dipole power: {power:.3}"),
        ];

        text.0 = strings.join("\n");
    }

    Ok(())
}

fn visualize_directivity(
    source: Single<(&SampleEffects, &GlobalTransform), With<SamplePlayer>>,
    effects: Query<&SpatializerNode>,
    mut gizmos: Gizmos,
) -> Result {
    let (player, transform) = *source;
    let effect = effects.get_effect(player)?;
    let model = effect.simulator_settings.directivity;

    let coordinates = phonon::scene::coordinate_space::CoordinateSpace3f {
        right: *transform.right(),
        up: *transform.up(),
        ahead: *transform.forward(),
        origin: transform.translation(),
    };

    const NUM_SAMPLES: usize = 30;
    const DT: f32 = 2.0 * PI / (NUM_SAMPLES as f32);
    let sample_positions: Vec<Vec3> = (0..NUM_SAMPLES)
        .map(|i| Vec3 {
            x: (i as f32 * DT).cos(),
            y: 0.2,
            z: (i as f32 * DT).sin(),
        })
        .collect();

    // Evaluate the directivity at each sample position
    let evaluated: Vec<f32> = sample_positions
        .iter()
        .map(|position| model.evaluate_at(*position, &coordinates))
        .collect();

    // Create a line from each sample to the next
    for i in 0..(NUM_SAMPLES - 1) {
        let start = evaluated[i] * sample_positions[i];
        let end = evaluated[i + 1] * sample_positions[i + 1];
        gizmos.line(start, end, RED);
    }

    // Close the loop
    let start = evaluated[NUM_SAMPLES - 1] * sample_positions[NUM_SAMPLES - 1];
    let end = evaluated[0] * sample_positions[0];
    gizmos.line(start, end, RED);

    Ok(())
}
