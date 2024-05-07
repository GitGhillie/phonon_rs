use phonon::direct_effect::DirectEffectParameters;
use phonon::panning_effect::PanningEffectParameters;

pub mod builder;
pub(crate) mod effect;
pub mod handle;

pub(crate) enum Command {
    SetParameters(DirectEffectParameters),
    SetPanning(PanningEffectParameters),
}
