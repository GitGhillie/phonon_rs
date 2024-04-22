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

const NUM_INTERPOLATION_FRAMES: usize = 4;

pub struct GainEffectParameters {
    pub gain: f32,
}

pub struct GainEffect {
    frame_size: usize,
    previous_gain: f32,
    first_frame: bool,
}

impl GainEffect {
    pub fn new(audio_settings: AudioSettings) -> Self {
        Self {
            frame_size: audio_settings.frame_size,
            previous_gain: 0.0,
            first_frame: true,
        }
    }

    pub(crate) fn reset(&mut self) {
        self.previous_gain = 0.0;
        self.first_frame = true;
    }

    pub fn apply(
        &mut self,
        parameters: GainEffectParameters,
        input: &AudioBuffer<1>,
        output: &mut AudioBuffer<1>,
    ) -> AudioEffectState {
        //todo: in and out length must be equal

        if self.first_frame {
            output.0[0] = input.0[0].iter().map(|x| x * parameters.gain).collect();
            self.previous_gain = parameters.gain;
            self.first_frame = false;
        } else {
            let target_gain = self.previous_gain
                + (1.0 / NUM_INTERPOLATION_FRAMES as f32) * (parameters.gain - self.previous_gain);

            let mut current_gain = self.previous_gain;
            let d_gain = (target_gain - self.previous_gain) / self.frame_size as f32;

            output.0[0] = input.0[0]
                .iter()
                .map(|x| {
                    let sample = current_gain * x;
                    current_gain += d_gain;
                    sample
                })
                .collect();

            self.previous_gain = target_gain;
        }

        AudioEffectState::TailComplete
    }

    pub(crate) fn tail_apply(
        &mut self,
        input: &AudioBuffer<1>,
        output: &mut AudioBuffer<1>,
    ) -> AudioEffectState {
        let previous_params = GainEffectParameters {
            gain: self.previous_gain,
        };

        return self.apply(previous_params, input, output);
    }

    pub(crate) fn tail(output: &mut AudioBuffer<1>) -> AudioEffectState {
        output.make_silent();
        AudioEffectState::TailComplete
    }
}
