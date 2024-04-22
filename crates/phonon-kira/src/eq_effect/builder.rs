use super::effect::EqEffectWrapped;
use super::handle::EqEffectHandle;
use kira::track::effect::{Effect, EffectBuilder};
use ringbuf::HeapRb;

const COMMAND_CAPACITY: usize = 8;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct EqEffectBuilder {
    pub eq_gains: [f32; 3],
    pub gain: f32,
}

impl EffectBuilder for EqEffectBuilder {
    type Handle = EqEffectHandle;

    fn build(self) -> (Box<dyn Effect>, Self::Handle) {
        let (command_producer, command_consumer) = HeapRb::new(COMMAND_CAPACITY).split();
        (
            Box::new(EqEffectWrapped::new(self, command_consumer)),
            EqEffectHandle { command_producer },
        )
    }
}
