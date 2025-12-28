use bevy::{color::palettes::css::ORANGE_RED, prelude::*};
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
            (rotate_cube, controls, update_ui).run_if(in_state(SceneSelection::Directivity)),
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
        Mesh3d(meshes.add(Cuboid::from_length(0.25))),
        MeshMaterial3d(materials.add(Color::srgb_u8(255, 144, 124))),
        Transform::from_xyz(0.0, 1.5, 0.0),
        DespawnOnExit(SceneSelection::Directivity),
    ));
}

fn setup_ui(mut commands: Commands) {
    // todo: Example description
    // let text = String::from_utf8(TextAssets::get("intro.md").unwrap().data.to_vec());
    // commands.spawn((
    //     DespawnOnExit(SceneSelection::Directivity),
    //     Text::from(text.unwrap()),
    //     text_shadow_component(),
    //     Node {
    //         position_type: PositionType::Absolute,
    //         bottom: px(5),
    //         left: px(15),
    //         ..default()
    //     },
    // ));

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
    time: Res<Time<Real>>,
) -> Result {
    let mut effect = effects.get_effect_mut(&player)?;
    let sensitivity = 0.5;

    if keyboard_input.just_pressed(KeyCode::Digit1) {
        effect.direct_effect_parameters.flags.directivity ^= true;
    }
    if keyboard_input.pressed(KeyCode::Digit2) {
        effect.simulator_settings.directivity.dipole_power -= sensitivity * time.delta_secs();
    }
    if keyboard_input.pressed(KeyCode::Digit3) {
        effect.simulator_settings.directivity.dipole_power += sensitivity * time.delta_secs();
    }
    if keyboard_input.pressed(KeyCode::Digit4) {
        effect.simulator_settings.directivity.dipole_weight -= sensitivity * time.delta_secs();
    }
    if keyboard_input.pressed(KeyCode::Digit5) {
        effect.simulator_settings.directivity.dipole_weight += sensitivity * time.delta_secs();
    }

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
        let strings = vec![
            "Sound sources can emit sound with different intensities in different directions."
                .to_string(),
            "Press the following keys to affect the directivity:".to_string(),
            format!("[1] - Directivity enabled: {directivity}"),
            format!("[2/3] - Dipole weight: {weight}"),
            format!("[4/5] - Dipole power: {power}"),
        ];

        text.0 = strings.join("\n");
    }

    Ok(())
}
