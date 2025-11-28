use bevy::prelude::*;

use crate::{
    AssetLoadingState,
    scenes::{DemoScene, SceneSelection, distance_effects::DistanceEffectsDemo, intro::IntroDemo},
};

#[derive(Debug)]
pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        let intro = IntroDemo;
        let distance_effects_demo = DistanceEffectsDemo;

        app.add_sub_state::<SceneSelection>();
        app.add_systems(
            Update,
            select_scene.run_if(in_state(AssetLoadingState::Loaded)),
        );

        intro.setup_systems(app, OnEnter(SceneSelection::Intro));
        distance_effects_demo.setup_systems(app, OnEnter(SceneSelection::DistanceAttenuation));
        distance_effects_demo.update_systems(app, Update);
    }
}

fn select_scene(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<SceneSelection>>,
    mut next_state: ResMut<NextState<SceneSelection>>,
) {
    if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        *next_state = bevy::prelude::NextState::Pending(state.next());
    }
    if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        *next_state = bevy::prelude::NextState::Pending(state.previous());
    }
}
