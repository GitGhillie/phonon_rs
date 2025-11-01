use crate::AssetLoadingState;
use bevy::{
    ecs::{schedule::ScheduleLabel, system::ScheduleSystem},
    prelude::*,
};

pub mod distance_effects;
pub mod intro;

pub(crate) trait DemoScene {
    fn setup_systems(&self, app: &mut App, schedule: impl ScheduleLabel);
    fn update_systems(&self, app: &mut App, schedule: impl ScheduleLabel) {}
}

#[derive(SubStates, Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(AssetLoadingState = AssetLoadingState::Loaded)]
pub(crate) enum SceneSelection {
    #[default]
    Intro,
    DistanceAttenuation,
}

impl SceneSelection {
    const SEQUENCE: [SceneSelection; 2] =
        [SceneSelection::Intro, SceneSelection::DistanceAttenuation];

    pub fn next(self) -> Self {
        let current_scene_index = Self::SEQUENCE.iter().position(|s| *s == self).unwrap();
        let next_scene_index = (current_scene_index + 1) % Self::SEQUENCE.len();
        Self::SEQUENCE[next_scene_index]
    }
}
