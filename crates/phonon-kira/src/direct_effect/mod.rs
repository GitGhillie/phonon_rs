use kira::command_writers_and_readers;
use phonon::direct_effect::DirectEffectParameters;
use phonon::panning_effect::PanningEffectParameters;

pub mod builder;
pub(crate) mod effect;
pub mod handle;

command_writers_and_readers!(
  set_parameters: DirectEffectParameters,
    set_panning: PanningEffectParameters,
);
