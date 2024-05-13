use super::command_writers_and_readers;
use super::effect::EqEffectWrapped;
use super::handle::EqEffectHandle;
use kira::effect::{Effect, EffectBuilder};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct EqEffectBuilder {
    pub eq_gains: [f32; 3],
    pub gain: f32,
}

impl EffectBuilder for EqEffectBuilder {
    type Handle = EqEffectHandle;

    fn build(self) -> (Box<dyn Effect>, Self::Handle) {
        let (command_writers, command_readers) = command_writers_and_readers();
        (
            Box::new(EqEffectWrapped::new(self, command_readers)),
            EqEffectHandle { command_writers },
        )
    }
}
