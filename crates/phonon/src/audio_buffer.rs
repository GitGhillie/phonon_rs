//
// Copyright 2017-2023 Valve Corporation.
// Copyright 2024 phonon_rs contributors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use derive_deref::{Deref, DerefMut};

pub enum AudioEffectState {
    TailRemaining,
    TailComplete,
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
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
pub struct AudioBuffer<const N_CHANNELS: usize>(pub [Vec<f32>; N_CHANNELS]);

impl<const N_CHANNELS: usize> AudioBuffer<N_CHANNELS> {
    pub fn new(num_samples: usize) -> Self {
        AudioBuffer(core::array::from_fn(|_| vec![0.0; num_samples]))
    }

    pub fn make_silent(&mut self) {
        for channel in &mut self.0 {
            channel.fill(0.0);
        }
    }

    // todo perf?
    pub fn mix(&mut self, other: &AudioBuffer<N_CHANNELS>) {
        for i in 0..other.len() {
            for j in 0..other[0].len() {
                self[i][j] += other[i][j];
            }
        }
    }

    // todo perf?
    /// Combine and average samples from all channels into a single one.
    pub fn downmix(&self, other: &mut AudioBuffer<1>) {
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
    /// todo: Can panic if the length of `other` is too small.
    pub fn write(&mut self, other: &[f32]) {
        let mut index = 0;

        for i in 0..self[0].len() {
            for j in 0..N_CHANNELS {
                self[j][i] = other[index];
                index += 1;
            }
        }
    }

    /// Converts `AudioBuffer` to interleaved slice.
    /// todo: Check perf?
    /// todo: Can panic if the length of `other` is too small.
    pub fn read(&self, other: &mut [f32]) {
        let mut index = 0;

        for i in 0..self[0].len() {
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
