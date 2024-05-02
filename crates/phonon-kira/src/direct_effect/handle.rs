use super::Command;
use kira::CommandError;
use phonon::direct_effect::DirectEffectParameters;
use phonon::panning_effect::PanningEffectParameters;
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

    pub fn set_panning(&mut self, params: PanningEffectParameters) -> Result<(), CommandError> {
        self.command_producer
            .push(Command::SetPanning(params))
            .map_err(|_| CommandError::CommandQueueFull)
    }
}
