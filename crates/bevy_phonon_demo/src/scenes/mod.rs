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
