use phonon::direct_effect::DirectEffectParameters;

pub mod builder;
pub(crate) mod effect;
pub mod handle;

pub(crate) enum Command {
    SetParameters(DirectEffectParameters),
}
