use super::Command;
use kira::CommandError;
use ringbuf::HeapProducer;

/// Controls an EQ effect.
pub struct EqEffectHandle {
    pub(super) command_producer: HeapProducer<Command>,
}

impl EqEffectHandle {
    pub fn set_gains(&mut self, gains: [f32; 3]) -> Result<(), CommandError> {
        self.command_producer
            .push(Command::SetGains(gains))
            .map_err(|_| CommandError::CommandQueueFull)
    }
}
