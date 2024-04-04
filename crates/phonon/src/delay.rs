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

// Port notes:
// - `Delay` is just a ring buffer implementation
// - Skipped implementing `resize()` as it does not seem to be used in Steam Audio

pub struct Delay {
    ring_buffer: Vec<f32>,
    cursor: usize,
    read_cursor: usize,
}

impl Delay {
    pub fn new(delay: usize, frame_size: usize) -> Self {
        Self {
            ring_buffer: Vec::with_capacity(delay + frame_size),
            cursor: 0,
            read_cursor: 0,
        }
    }

    pub fn reset(&mut self) {
        self.ring_buffer.fill_with(Default::default);
    }

    // `out` slice length and num_samples must be the same
    pub fn get(&mut self, num_samples: usize, out: &mut [f32]) {
        if self.read_cursor + (num_samples - 1) < self.ring_buffer.len() {
            out.copy_from_slice(
                &self.ring_buffer[self.read_cursor..self.read_cursor + num_samples],
            );

            self.read_cursor += num_samples;
            if self.read_cursor > self.ring_buffer.len() {
                self.read_cursor -= self.ring_buffer.len();
            }
        } else {
            let size1 = self.ring_buffer.len() - self.read_cursor;
            let size2 = num_samples - size1;

            out[..size1].copy_from_slice(&self.ring_buffer[self.read_cursor..]);
            out[size1..].copy_from_slice(&self.ring_buffer[..size2]);

            self.read_cursor = size2;
        }
    }

    pub fn put(mut self, num_samples: usize, input: &[f32]) {
        if self.cursor + (num_samples - 1) < self.ring_buffer.len() {
            self.ring_buffer[self.cursor..self.cursor + num_samples].copy_from_slice(input);
            self.cursor += num_samples;
            if self.cursor > self.ring_buffer.len() {
                self.cursor -= self.ring_buffer.len();
            }
        } else {
            let size1 = self.ring_buffer.len() - self.cursor;
            let size2 = num_samples - size1;

            self.ring_buffer[self.cursor..].copy_from_slice(&input[..size1]);
            self.ring_buffer[..size2].copy_from_slice(&input[size2..]);

            self.cursor = size2;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delay_buffer() {
        let mut delay = Delay {
            ring_buffer: vec![1.0, 2.0, 3.0],
            cursor: 0,
            read_cursor: 0,
        };

        assert_eq!(2.0, delay.ring_buffer[1]);
        assert_eq!(3, delay.ring_buffer.len());
    }
}
