use super::CommandWriters;

/// Controls an EQ effect.
pub struct EqEffectHandle {
    pub(super) command_writers: CommandWriters,
}

impl EqEffectHandle {
    pub fn set_eq_gains(&mut self, gains: [f32; 3]) {
        self.command_writers.set_eq_gains.write(gains);
    }

    pub fn set_gain(&mut self, gain: f32) {
        self.command_writers.set_gain.write(gain);
    }
}
