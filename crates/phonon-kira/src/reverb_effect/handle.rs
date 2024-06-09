use super::CommandWriters;

/// Controls an EQ effect.
pub struct ReverbEffectHandle {
    pub(super) command_writers: CommandWriters,
}

impl ReverbEffectHandle {
    pub fn set_reverb_times(&mut self, times: [f32; 3]) {
        self.command_writers.set_reverb_times.write(times);
    }

    pub fn set_wet(&mut self, wet: bool) {
        self.command_writers.set_wet.write(wet);
    }

    pub fn set_dry(&mut self, dry: bool) {
        self.command_writers.set_dry.write(dry);
    }
}
