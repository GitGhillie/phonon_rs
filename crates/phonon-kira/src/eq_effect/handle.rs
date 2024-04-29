use super::Command;
use kira::CommandError;
use ringbuf::HeapProducer;

/// Controls an EQ effect.
pub struct EqEffectHandle {
    pub(super) command_producer: HeapProducer<Command>,
}

impl EqEffectHandle {
    pub fn set_eq_gains(&mut self, gains: [f32; 3]) -> Result<(), CommandError> {
        self.command_producer
            .push(Command::SetEqGains(gains))
            .map_err(|_| CommandError::CommandQueueFull)
    }

    pub fn set_gain(&mut self, gain: f32) -> Result<(), CommandError> {
        self.command_producer
            .push(Command::SetGain(gain))
            .map_err(|_| CommandError::CommandQueueFull)
    }
}
