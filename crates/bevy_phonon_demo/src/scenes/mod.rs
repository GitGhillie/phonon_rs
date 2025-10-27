use bevy::state::state::States;

pub mod intro;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum SceneSelection {
    #[default]
    Intro,
    DistanceAttenuation,
}
