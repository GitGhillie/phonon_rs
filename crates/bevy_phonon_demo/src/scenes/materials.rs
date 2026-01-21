use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::*;
use bevy_phonon::{effects::spatializer::SpatializerNode, prelude::*};
use bevy_seedling::{
    prelude::{EffectsQuery, SampleEffects},
    sample::SamplePlayer,
    sample_effects,
};
use phonon::effects::direct::TransmissionType;

use crate::{
    DemoAssets,
    scenes::{DemoScene, SceneSelection, text_shadow_component},
};

pub(crate) struct MaterialsDemo;

impl DemoScene for MaterialsDemo {
    fn setup_systems(&self, app: &mut App, schedule: impl ScheduleLabel) {
        app.add_systems(schedule, (setup, setup_ui));
    }

    fn update_systems(&self, app: &mut App, schedule: impl ScheduleLabel) {
        app.add_systems(
            schedule,
            (controls, update_ui).run_if(in_state(SceneSelection::Materials)),
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
        Mesh3d(meshes.add(Sphere::new(0.2))),
        MeshMaterial3d(materials.add(Color::srgb_u8(233, 1, 1))),
        Transform::from_xyz(0.0, 1.5, -1.2),
        DespawnOnExit(SceneSelection::Materials),
    ));

    commands.spawn((
        Name::from("Audio Geometry"),
        Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 0.1))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba_u8(1, 1, 233, 137),
            alpha_mode: AlphaMode::Blend,
            ..default()
        })),
        NeedsAudioMesh(materials::CARPET),
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
        effect.direct_effect_parameters.flags.transmission ^= true;
    }

    if keyboard_input.just_pressed(KeyCode::Digit2) {
        if effect.direct_effect_parameters.transmission_type == TransmissionType::FrequencyDependent
        {
            effect.direct_effect_parameters.transmission_type =
                TransmissionType::FrequencyIndependent
        } else {
            effect.direct_effect_parameters.transmission_type = TransmissionType::FrequencyDependent
        }
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
        let strings = [
            "Transmission determines how sound is absorbed by a material".to_string(),
            "With frequency dependent transmission some frequencies may be absorbed quicker than others".to_string(),
            format!("[1] - Transmission: {transmission}"),
            format!("[2] - Transmission type: {transmission_type:?}"),
        ];

        text.0 = strings.join("\n");
    }

    Ok(())
}
