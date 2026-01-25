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
    pub sampling_rate: u32,
    pub frame_size: usize,
}

impl AudioSettings {
    pub fn new(sampling_rate: u32, frame_size: usize) -> Self {
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
/// written from using [`AudioBufferMut::read_interleaved`] and [`AudioBuffer::write_interleaved`].
pub trait AudioBuffer {
    /// Returns the number of channels this `AudioBuffer` has.
    fn num_channels(&self) -> usize;
    /// Returns the number of samples each channel has.
    fn num_samples(&self) -> usize;
    /// Mixes the `AudioBuffer` into another by adding samples together.
    fn mix(&self, other: &mut [&mut [f32]]);
    /// Mixes all channels on an `AudioBuffer` into a single output channel.
    /// Downmixing is performed by summing up the source channels and dividing
    /// the result by the number of source channels.
    fn downmix(&self, output: &mut [&mut [f32]]);
    /// Writes the `AudioBuffer` to an interleaved slice.
    fn write_interleaved(&self, target: &mut [&mut [f32]]);
}

pub trait AudioBufferMut {
    /// Returns the number of channels this `AudioBuffer` has.
    fn num_channels(&self) -> usize;
    /// Returns the number of samples each channel has.
    fn num_samples(&self) -> usize;
    /// Fills the `AudioBuffer` with all zero samples, representing silence.
    fn make_silent(&mut self);
    /// Reads a slice of interleaved samples into this `AudioBuffer`.
    fn read_interleaved(&mut self, source: &[&[f32]]);
    /// Scales all the samples in the `AudioBuffer` by the given volume.
    fn scale(&mut self, volume: f32);
}

/// Owned audio buffer, deinterleaved
#[derive(Deref, DerefMut)]
pub struct ScratchBuffer(Vec<Vec<f32>>);

impl ScratchBuffer {
    /// Creates a new `ScratchBuffer` with a fixed number of channels and samples.
    ///
    /// Initalized to all zeros, representing silence.
    pub fn new(num_channels: usize, num_samples: usize) -> Self {
        Self(vec![vec![0.0; num_samples]; num_channels])
    }

    pub fn as_ref<'a>(&'a self) -> Vec<&'a [f32]> {
        self.iter().map(|channel| channel.as_slice()).collect()
    }

    pub fn as_ref_mut<'a>(&'a mut self) -> Vec<&'a mut [f32]> {
        self.iter_mut()
            .map(|channel| channel.as_mut_slice())
            .collect()
    }
}

impl AudioBufferMut for [&mut [f32]] {
    #[inline]
    fn num_channels(&self) -> usize {
        self.len()
    }

    #[inline]
    fn num_samples(&self) -> usize {
        self[0].len()
    }

    fn make_silent(&mut self) {
        for channel in self {
            channel.fill(0.0);
        }
    }

    // todo: Check perf?
    // todo: Can panic if the length of `other` is too small.
    fn read_interleaved(&mut self, source: &[&[f32]]) {
        let mut index = 0;

        for i in 0..self.num_samples() {
            for j in 0..self.num_channels() {
                self[j][i] = source[0][index];
                index += 1;
            }
        }
    }

    // todo: Check perf?
    fn scale(&mut self, volume: f32) {
        for i in 0..self.num_channels() {
            for j in 0..self.num_samples() {
                self[i][j] *= volume;
            }
        }
    }
}

impl AudioBuffer for [&[f32]] {
    #[inline]
    fn num_channels(&self) -> usize {
        self.len()
    }

    #[inline]
    fn num_samples(&self) -> usize {
        self[0].len()
    }

    // todo perf?
    fn mix(&self, other: &mut [&mut [f32]]) {
        for i in 0..other.num_channels() {
            for j in 0..other.num_samples() {
                other[i][j] += self[i][j];
            }
        }
    }

    // todo perf?
    fn downmix(&self, output: &mut [&mut [f32]]) {
        let num_channels = self.num_channels();
        let factor = 1.0 / (num_channels as f32);

        for i in 0..output.num_samples() {
            let mut sum = 0.0;

            for j in 0..num_channels {
                sum += self[j][i];
            }

            output[0][i] = sum * factor;
        }
    }

    // todo: Check perf?
    // todo: Can panic if the length of `other` is too small.
    fn write_interleaved(&self, target: &mut [&mut [f32]]) {
        let mut index = 0;

        for i in 0..self.num_samples() {
            for j in 0..self.num_channels() {
                target[0][index] = self[j][i];
                index += 1;
            }
        }
    }
}
