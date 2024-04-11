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

use crate::audio_buffer::{AudioBuffer, AudioEffectState, AudioSettings};
use ndarray::{Array, Array1};

use crate::bands::{HIGH_CUTOFF_FREQUENCIES, LOW_CUTOFF_FREQUENCIES, NUM_BANDS};
use crate::iir::{IIRFilterer, IIR};

struct EqEffectParameters {
    gains: [f32; NUM_BANDS],
}

pub struct EqEffect {
    sampling_rate: i32,
    frame_size: usize,
    /// Two rows of filterers, one for the current `EqEffectParameters` and one for the previous
    /// `EqEffectParameters`. Which row is which depends on the `current` field.
    filters: [[IIRFilterer; 2]; NUM_BANDS],
    /// If the `EqEffectParameters` change this array is filled with samples processed by the
    /// previous filters, in order to be able to transition smoothly.
    temp: Array1<f32>,
    /// Gains from the previous filters, in case the `EqEffectParameters` change.
    previous_gains: [f32; NUM_BANDS],
    /// Current row of `filters` that is applicable.
    current: usize,
}

impl EqEffect {
    pub fn new(audio_settings: AudioSettings) -> Self {
        let mut eq_effect = Self {
            sampling_rate: audio_settings.sampling_rate,
            frame_size: audio_settings.frame_size,
            filters: [[IIRFilterer::new(IIR::new_empty()); 2]; NUM_BANDS],
            temp: Array::zeros(audio_settings.frame_size), // Doesn't need to be zeros
            previous_gains: [1.0, 1.0, 1.0],
            current: 0,
        };

        eq_effect.set_filter_gains(0, [0.0, 0.0, 0.0].as_slice());

        eq_effect
    }

    fn reset(&mut self) {
        for band in 0..NUM_BANDS {
            self.previous_gains[band] = 1.0;
        }

        let gains = self.previous_gains.clone();
        self.set_filter_gains(0, gains.as_slice());

        self.current = 0;
    }

    fn apply(
        &mut self,
        parameters: EqEffectParameters,
        input: &AudioBuffer<1>,
        output: &mut AudioBuffer<1>,
    ) -> AudioEffectState {
        //todo: Function can panic if `output` is too short

        // If any of the gains change, the filters also need to change.
        // If the filters change, we need to use the previous filters to
        // create a smooth transition.
        if self.previous_gains[0] != parameters.gains[0]
            || self.previous_gains[1] != parameters.gains[1]
            || self.previous_gains[2] != parameters.gains[2]
        {
            let previous = self.current;
            self.current = 1 - self.current;

            self.set_filter_gains(self.current, parameters.gains.as_slice());

            self.filters[0][self.current].copy_state_from(self.filters[0][previous]);
            self.filters[1][self.current].copy_state_from(self.filters[1][previous]);
            self.filters[2][self.current].copy_state_from(self.filters[2][previous]);

            self.apply_filter_to_temp_cascade(previous, &input[0]);
            self.apply_filter_cascade(self.current, &input[0], &mut output[0]);

            for i in 0..self.frame_size {
                let weight = (i / self.frame_size) as f32;
                output[0][i] = weight * output[0][i] + (1.0 - weight) * self.temp[i];
            }

            for i in 0..NUM_BANDS {
                self.previous_gains[i] = parameters.gains[i];
            }
        } else {
            self.apply_filter_cascade(self.current, &input[0], &mut output[0]);
        }

        return AudioEffectState::TailComplete;
    }

    fn tail_apply(
        &mut self,
        input: &AudioBuffer<1>,
        output: &mut AudioBuffer<1>,
    ) -> AudioEffectState {
        self.apply(
            EqEffectParameters {
                gains: self.previous_gains,
            },
            input,
            output,
        )
    }

    fn tail(output: &mut AudioBuffer<1>) -> AudioEffectState {
        output.make_silent();
        AudioEffectState::TailComplete
    }

    fn set_filter_gains(&mut self, index: usize, gains: &[f32]) {
        self.filters[0][index].set_filter(IIR::new_low_shelf(
            HIGH_CUTOFF_FREQUENCIES[0],
            gains[0],
            self.sampling_rate,
        ));

        self.filters[1][index].set_filter(IIR::new_peaking(
            LOW_CUTOFF_FREQUENCIES[1],
            HIGH_CUTOFF_FREQUENCIES[1],
            gains[1],
            self.sampling_rate,
        ));

        self.filters[2][index].set_filter(IIR::new_high_shelf(
            LOW_CUTOFF_FREQUENCIES[2],
            gains[2],
            self.sampling_rate,
        ));
    }

    fn apply_filter_cascade(&mut self, index: usize, input: &[f32], output: &mut [f32]) {
        self.filters[0][index].apply(self.frame_size, input, output);
        self.filters[1][index].apply(self.frame_size, input, output);
        self.filters[2][index].apply(self.frame_size, input, output);
    }

    fn apply_filter_to_temp_cascade(&mut self, index: usize, input: &[f32]) {
        self.filters[0][index].apply(self.frame_size, input, self.temp.as_slice_mut().unwrap());
        self.filters[1][index].apply(self.frame_size, input, self.temp.as_slice_mut().unwrap());
        self.filters[2][index].apply(self.frame_size, input, self.temp.as_slice_mut().unwrap());
    }
}
