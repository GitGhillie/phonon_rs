use super::command_writers_and_readers;
use super::effect::ReverbEffectWrapped;
use super::handle::ReverbEffectHandle;
use kira::effect::{Effect, EffectBuilder};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ReverbEffectBuilder {
    pub reverb_times: [f32; 3],
    pub dry: bool,
    pub wet: bool,
}

impl EffectBuilder for ReverbEffectBuilder {
    type Handle = ReverbEffectHandle;

    fn build(self) -> (Box<dyn Effect>, Self::Handle) {
        let (command_writers, command_readers) = command_writers_and_readers();
        (
            Box::new(ReverbEffectWrapped::new(self, command_readers)),
            ReverbEffectHandle { command_writers },
        )
    }
}
