use crate::AssetLoadingState;
use bevy::prelude::StateSet;
use bevy::state::state::SubStates;

pub mod intro;

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(AssetLoadingState = AssetLoadingState::Loaded)]
pub(crate) enum SceneSelection {
    #[default]
    Intro,
    DistanceAttenuation,
}

impl SceneSelection {
    const SEQUENCE: [SceneSelection; 2] =
        [SceneSelection::Intro, SceneSelection::DistanceAttenuation];

    fn next(self) -> Self {
        let i = Self::SEQUENCE.iter().position(|s| *s == self).unwrap();
        Self::SEQUENCE[(i + 1) % Self::SEQUENCE.len()]
    }
}
