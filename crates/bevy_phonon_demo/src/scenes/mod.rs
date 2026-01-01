use crate::AssetLoadingState;
use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
use rust_embed::Embed;

pub mod binaural;
pub mod directivity;
pub mod distance_effects;
pub mod intro;
pub mod materials;

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
    Directivity,
}

const NUM_SCENES: usize = 3;

impl SceneSelection {
    const SEQUENCE: [SceneSelection; NUM_SCENES] = [
        SceneSelection::Intro,
        SceneSelection::DistanceAttenuation,
        SceneSelection::Directivity,
    ];

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

pub(crate) fn text_shadow_component() -> TextShadow {
    TextShadow {
        offset: Vec2 { x: 2.0, y: 2.0 },
        ..Default::default()
    }
}
