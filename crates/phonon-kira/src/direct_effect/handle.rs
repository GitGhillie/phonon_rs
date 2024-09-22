use super::CommandWriters;
use phonon::effects::direct::DirectEffectParameters;
use phonon::effects::panning::PanningEffectParameters;

/// Controls an EQ effect.
pub struct DirectEffectHandle {
    pub(super) command_writers: CommandWriters,
}

impl DirectEffectHandle {
    pub fn set_parameters(&mut self, params: DirectEffectParameters) {
        self.command_writers.set_parameters.write(params);
    }

    pub fn set_panning(&mut self, params: PanningEffectParameters) {
        self.command_writers.set_panning.write(params);
    }
}
