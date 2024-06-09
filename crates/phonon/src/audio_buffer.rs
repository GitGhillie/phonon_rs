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

#[derive(Debug, Copy, Clone, PartialEq)]
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

/// Phonon processes audio in audio buffers, which contain uncompressed Pulse
/// Code Modulated (PCM) data (just like a .wav file).
///
/// Audio buffers contain one or more channels; for example, a stereo audio
/// buffer contains 2 channels. Each channel typically contains several samples,
/// which are values of the audio signal’s level at discrete points of time.
/// Each channel has the same number of samples.
///
/// A sample is a 32-bit floating-point. The time interval between two
/// successive samples is specified using the sampling rate. Typical sampling
/// rates are 44100 Hz (CD quality) or 48000 Hz.
///
/// # Interleaving
/// Phonon stores audio buffers in a deinterleaved layout. This means that all
/// the samples for the first channel are store contiguously, followed by all
/// the samples for the second channel, and so on.
///
/// Most audio formats instead use an interleaved layout, where data for each
/// frame is stored together in memory. Interleaved data can be read to and
/// written from using [`AudioBuffer::read_interleaved`] and [`AudioBuffer::write_interleaved`].
#[derive(Deref, DerefMut)]
pub struct AudioBuffer<const N_CHANNELS: usize>(pub [Vec<f32>; N_CHANNELS]);

impl<const N_CHANNELS: usize> AudioBuffer<N_CHANNELS> {
    /// Creates a new `AudioBuffer` with a fixed number of channels and samples.
    ///
    /// Initalized to all zeros, representing silence.
    pub fn new(num_samples: usize) -> Self {
        AudioBuffer(core::array::from_fn(|_| vec![0.0; num_samples]))
    }

    /// Returns the number of channels this `AudioBuffer` has.
    pub fn num_channels(&self) -> usize {
        self.len()
    }

    /// Returns the number of samples each channel has.
    pub fn num_samples(&self) -> usize {
        self[0].len()
    }

    /// Fills the `AudioBuffer` with all zero samples, representing silence.
    pub fn make_silent(&mut self) {
        for channel in &mut self.0 {
            channel.fill(0.0);
        }
    }

    /// Mixes the `AudioBuffer` into another by adding samples together.
    // todo perf?
    pub fn mix(&mut self, other: &AudioBuffer<N_CHANNELS>) {
        for i in 0..other.len() {
            for j in 0..other[0].len() {
                self[i][j] += other[i][j];
            }
        }
    }

    /// Mixes all channels on an `AudioBuffer` into a single output channel.
    /// Downmixing is performed by summing up the source channels and dividing
    /// the result by the number of source channels.
    // todo perf?
    pub fn downmix(&self, output: &mut AudioBuffer<1>) {
        let num_channels = self.len();
        let factor = 1.0 / (num_channels as f32);

        for i in 0..output[0].len() {
            let mut sum = 0.0;

            for j in 0..num_channels {
                sum += self[j][i];
            }

            output[0][i] = sum * factor;
        }
    }

    /// Reads a slice of interleaved samples into this `AudioBuffer`.
    // todo: Check perf?
    // todo: Can panic if the length of `other` is too small.
    pub fn read_interleaved(&mut self, source: &[f32]) {
        let mut index = 0;

        for i in 0..self[0].len() {
            for j in 0..N_CHANNELS {
                self[j][i] = source[index];
                index += 1;
            }
        }
    }

    /// Writes the `AudioBuffer` to an interleaved slice.
    // todo: Check perf?
    // todo: Can panic if the length of `other` is too small.
    pub fn write_interleaved(&self, target: &mut [f32]) {
        let mut index = 0;

        for i in 0..self[0].len() {
            for j in 0..N_CHANNELS {
                target[index] = self[j][i];
                index += 1;
            }
        }
    }

    /// Scales all the samples in the `AudioBuffer` by the given volume.
    // todo: Check perf?
    pub fn scale(&mut self, volume: f32) {
        for i in 0..self.len() {
            for j in 0..self[0].len() {
                self[i][j] *= volume;
            }
        }
    }
}
