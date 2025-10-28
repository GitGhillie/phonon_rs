use bevy::prelude::*;

use crate::scenes::{SceneSelection, intro::setup};

#[derive(Debug)]
pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<SceneSelection>()
            .add_systems(OnEnter(SceneSelection::Intro), setup);
    }
}
