use super::CommandWriters;
use phonon::effects::binaural::BinauralEffectParameters;
use phonon::effects::direct::DirectEffectParameters;

/// Controls an EQ effect.
pub struct DirectEffectHandle {
    pub(super) command_writers: CommandWriters,
}

impl DirectEffectHandle {
    pub fn set_parameters(&mut self, params: DirectEffectParameters) {
        self.command_writers.set_parameters.write(params);
    }

    pub fn set_direction(&mut self, params: BinauralEffectParameters) {
        self.command_writers.set_direction.write(params);
    }
}
