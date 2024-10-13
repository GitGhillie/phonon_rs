use super::command_writers_and_readers;
use super::effect::DirectEffectWrapped;
use super::handle::DirectEffectHandle;
use kira::effect::{Effect, EffectBuilder};
use phonon::effects::binaural::BinauralEffectParameters;
use phonon::effects::direct::DirectEffectParameters;

#[derive(Debug, Copy, Clone)]
pub struct DirectEffectBuilder {
    pub parameters: DirectEffectParameters,
    pub binaural_params: BinauralEffectParameters,
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
