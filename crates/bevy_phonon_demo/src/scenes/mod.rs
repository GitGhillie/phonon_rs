use crate::AssetLoadingState;
use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
use rust_embed::Embed;

pub mod distance_effects;
pub mod intro;

#[derive(Embed)]
#[folder = "assets/text/"]
pub(crate) struct TextAssets;

pub(crate) trait DemoScene {
    fn setup_systems(&self, app: &mut App, schedule: impl ScheduleLabel);
    fn update_systems(&self, _app: &mut App, _schedule: impl ScheduleLabel) {}
}

#[derive(SubStates, Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(AssetLoadingState = AssetLoadingState::Loaded)]
pub(crate) enum SceneSelection {
    #[default]
    Intro,
    DistanceAttenuation,
}

const NUM_SCENES: usize = 2;

impl SceneSelection {
    const SEQUENCE: [SceneSelection; NUM_SCENES] =
        [SceneSelection::Intro, SceneSelection::DistanceAttenuation];

    pub fn next(self) -> Self {
        let current_scene_index = Self::SEQUENCE.iter().position(|s| *s == self).unwrap();
        let next_scene_index = (current_scene_index + 1) % Self::SEQUENCE.len();
        Self::SEQUENCE[next_scene_index]
    }

    pub fn previous(self) -> Self {
        let current_scene_index = Self::SEQUENCE.iter().position(|s| *s == self).unwrap();
        let next_scene_index = if current_scene_index == 0 {
            NUM_SCENES - 1
        } else {
            (current_scene_index - 1) % Self::SEQUENCE.len()
        };
        Self::SEQUENCE[next_scene_index]
    }
}
