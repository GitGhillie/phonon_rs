use bevy::prelude::*;

use crate::{
    AssetLoadingState,
    scenes::{self, SceneSelection},
};

#[derive(Debug)]
pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<SceneSelection>()
            .add_systems(
                Update,
                select_scene.run_if(in_state(AssetLoadingState::Loaded)),
            )
            .add_systems(OnEnter(SceneSelection::Intro), scenes::intro::setup)
            .add_systems(
                OnEnter(SceneSelection::DistanceAttenuation),
                scenes::distance_effects::setup,
            );
    }
}

fn select_scene(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<SceneSelection>>,
    mut next_state: ResMut<NextState<SceneSelection>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        *next_state = bevy::prelude::NextState::Pending(state.next());
    }
}
