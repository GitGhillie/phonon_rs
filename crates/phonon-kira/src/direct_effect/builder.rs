use super::command_writers_and_readers;
use super::effect::DirectEffectWrapped;
use super::handle::DirectEffectHandle;
use kira::effect::{Effect, EffectBuilder};
use phonon::direct_effect::DirectEffectParameters;
use phonon::panning_effect::PanningEffectParameters;

#[derive(Debug, Copy, Clone)]
pub struct DirectEffectBuilder {
    pub parameters: DirectEffectParameters,
    pub panning_params: PanningEffectParameters,
}

impl EffectBuilder for DirectEffectBuilder {
    type Handle = DirectEffectHandle;

    fn build(self) -> (Box<dyn Effect>, Self::Handle) {
        let (command_writers, command_readers) = command_writers_and_readers();
        (
            Box::new(DirectEffectWrapped::new(self, command_readers)),
            DirectEffectHandle { command_writers },
        )
    }
}
