use super::Command;
use kira::CommandError;
use phonon::direct_effect::DirectEffectParameters;
use ringbuf::HeapProducer;

/// Controls an EQ effect.
pub struct DirectEffectHandle {
    pub(super) command_producer: HeapProducer<Command>,
}

impl DirectEffectHandle {
    pub fn set_parameters(&mut self, params: DirectEffectParameters) -> Result<(), CommandError> {
        self.command_producer
            .push(Command::SetParameters(params))
            .map_err(|_| CommandError::CommandQueueFull)
    }
}
