use derive_deref::{Deref, DerefMut};

pub enum AudioEffectState {
    TailRemaining,
    TailComplete,
}

#[derive(Default)]
pub struct AudioSettings {
    pub sampling_rate: i32,
    pub frame_size: usize,
}

impl AudioSettings {
    pub fn new(sampling_rate: i32, frame_size: usize) -> Self {
        Self {
            sampling_rate,
            frame_size,
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct AudioBuffer<const N_CHANNELS: usize, const N_SAMPLES: usize>(
    pub [[f32; N_SAMPLES]; N_CHANNELS],
);

impl<const N_CHANNELS: usize, const N_SAMPLES: usize> Default
    for AudioBuffer<N_CHANNELS, N_SAMPLES>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<const N_CHANNELS: usize, const N_SAMPLES: usize> AudioBuffer<N_CHANNELS, N_SAMPLES> {
    pub fn new() -> Self {
        AudioBuffer([[0.0; N_SAMPLES]; N_CHANNELS])
    }

    pub fn make_silent(&mut self) {
        self.fill([0.0; N_SAMPLES]);
    }

    // todo perf?
    pub fn mix(&mut self, other: &AudioBuffer<N_CHANNELS, N_SAMPLES>) {
        for i in 0..other.len() {
            for j in 0..other[0].len() {
                self[i][j] += other[i][j];
            }
        }
    }

    // todo perf?
    /// Combine and average samples from all channels into a single one.
    pub fn downmix(&self, other: &mut AudioBuffer<1, N_SAMPLES>) {
        let num_channels = self.len();
        let factor = 1.0 / (num_channels as f32);

        for i in 0..other[0].len() {
            let mut sum = 0.0;

            for j in 0..num_channels {
                sum += self[j][i];
            }

            other[0][i] = sum * factor;
        }
    }

    /// Writes interleaved slice to `AudioBuffer`.
    /// todo: Check perf?
    pub fn write(&mut self, other: &[f32; N_CHANNELS * N_SAMPLES]) {
        let mut index = 0;

        for i in 0..N_SAMPLES {
            for j in 0..N_CHANNELS {
                self[j][i] = other[index];
                index += 1;
            }
        }
    }

    /// Converts `AudioBuffer` to interleaved slice.
    /// todo: Check perf?
    pub fn read(&self, other: &mut [f32; N_CHANNELS * N_SAMPLES]) {
        let mut index = 0;

        for i in 0..N_SAMPLES {
            for j in 0..N_CHANNELS {
                other[index] = self[j][i];
                index += 1;
            }
        }
    }

    // todo perf?
    pub fn scale(&mut self, volume: f32) {
        for i in 0..self.len() {
            for j in 0..self[0].len() {
                self[i][j] *= volume;
            }
        }
    }
}
