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
    scenes::{DemoScene, SceneSelection, TextAssets, text_shadow_component},
};

pub(crate) struct MaterialsDemo;

impl DemoScene for MaterialsDemo {
    fn setup_systems(&self, app: &mut App, schedule: impl ScheduleLabel) {
        app.add_systems(schedule, (setup, setup_ui));
    }

    fn update_systems(&self, app: &mut App, schedule: impl ScheduleLabel) {
        app.add_systems(
            schedule,
            (move_cubes, controls, update_ui).run_if(in_state(SceneSelection::Materials)),
        );
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    demo_assets: Res<DemoAssets>,
) {
    info!("Setting up scene");
    commands.spawn((
        Name::from("Audio Source"),
        SamplePlayer::new(demo_assets.audio_sample.clone()).looping(),
        sample_effects![SpatializerNode::default()],
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(233, 1, 1))),
        Transform::from_xyz(0.0, 1.5, 0.0),
        DespawnOnExit(SceneSelection::Materials),
    ));
}

fn controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player: Single<&SampleEffects, With<SamplePlayer>>,
    mut effects: Query<&mut SpatializerNode>,
) -> Result {
    let mut effect = effects.get_effect_mut(&player)?;

    if keyboard_input.just_pressed(KeyCode::Digit1) {
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
    commands.spawn((
        DespawnOnExit(SceneSelection::Materials),
        Text::from("A"),
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

fn update_ui(
    player: Single<&SampleEffects, With<SamplePlayer>>,
    effects: Query<&SpatializerNode>,
    mut text: Query<&mut Text, With<Name>>,
) -> Result {
    let effect = effects.get_effect(&player)?;
    // todo add num transmission rays
    let transmission = effect.direct_effect_parameters.flags.transmission;
    let transmission_type = effect.direct_effect_parameters.transmission_type;

    if let Ok(mut text) = text.single_mut() {
        let strings = vec![
            "Transmission determines how sound is absorbed by a material".to_string(),
            "With frequency dependent transmission some frequencies may be absorbed quicker than others".to_string(),
            format!("[2] - Transmission: {transmission}"),
            format!("[3] - Transmission type: {transmission_type:?}"),
        ];

        text.0 = strings.join("\n");
    }

    Ok(())
}
