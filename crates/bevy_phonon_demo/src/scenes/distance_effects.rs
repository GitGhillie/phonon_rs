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
    scenes::{DemoScene, SceneSelection, text_shadow_component},
};

pub(crate) struct DistanceEffectsDemo;

impl DemoScene for DistanceEffectsDemo {
    fn setup_systems(&self, app: &mut App, schedule: impl ScheduleLabel) {
        app.add_systems(schedule, (setup, setup_ui));
    }

    fn update_systems(&self, app: &mut App, schedule: impl ScheduleLabel) {
        app.add_systems(
            schedule,
            (move_sources, controls, update_ui)
                .run_if(in_state(SceneSelection::DistanceAttenuation)),
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
        Name::from("Translating Audio Source"),
        SamplePlayer::new(demo_assets.audio_sample.clone()).looping(),
        sample_effects![SpatializerNode::default()],
        Mesh3d(meshes.add(Sphere::new(0.2))),
        MeshMaterial3d(materials.add(Color::srgb_u8(233, 1, 1))),
        Transform::from_xyz(0.0, 1.5, 0.0),
        MoveMarker,
        DespawnOnExit(SceneSelection::DistanceAttenuation),
    ));
}

fn move_sources(mut sources: Query<&mut Transform, With<MoveMarker>>, time: Res<Time>) {
    let period = 5.0;
    let position = (time.elapsed_secs() * core::f32::consts::TAU / period).sin() * 10.0;

    for mut source_tf in sources.iter_mut() {
        source_tf.translation.x = position;
    }
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
        DespawnOnExit(SceneSelection::DistanceAttenuation),
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
    let distance_attenuation = effect.direct_effect_parameters.flags.distance_attenuation;
    let air_absorption = effect.direct_effect_parameters.flags.air_absorption;
    let delay = effect.direct_effect_parameters.flags.delay;

    if let Ok(mut text) = text.single_mut() {
        let strings = [
            "There are a few effects purely based on distance.".to_string(),
            "Press the following keys to toggle the effects:".to_string(),
            format!("[1] - Distance attenuation: {distance_attenuation}"),
            format!("[2] - Air absorption: {air_absorption}"),
            format!("[3] - Delay: {delay}"),
        ];

        text.0 = strings.join("\n");
    }

    Ok(())
}
