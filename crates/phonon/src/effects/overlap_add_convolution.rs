//
// Copyright 2017-2023 Valve Corporation.
// Copyright 2026 phonon_rs contributors.
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

use std::sync::Arc;

use rustfft::{Fft, FftPlanner, num_complex::Complex};

use crate::dsp::audio_buffer::AudioSettings;

pub struct OverlapAddConvolutionEffectSettings {
    pub num_channels: u8,
    pub ir_size: usize,
}

// Port note: multipleInputs did not seem to be used, so it is not added here
pub struct OverlapAddConvolutionEffectParams {
    pub fft_impulse_response: Vec<u8>,
    //pub multiple_inputs: bool,
}

pub struct OverlapAddConvolutionEffect {
    num_channels: u8,
    impulse_response_size: usize,
    frame_size: usize,
    window: Vec<f32>,
    fft_forward: Arc<dyn Fft<f32>>,
    fft_inverse: Arc<dyn Fft<f32>>,
    windowed_dry: Vec<f32>,
    fft_windowed_dry: Vec<Complex<f32>>,
    dry: Vec<f32>,
    wet: Vec<f32>,
    fft_wet: Vec<Complex<f32>>,
    overlap: Vec<f32>,
    num_tail_samples_remaining: usize,
}

impl OverlapAddConvolutionEffect {
    fn new(
        audio_settings: AudioSettings,
        effect_settings: OverlapAddConvolutionEffectSettings,
    ) -> Self {
        let frame_size = audio_settings.frame_size;
        let window_size = frame_size + (frame_size / 4);
        let fft_size = window_size + effect_settings.ir_size - 1;

        // todo: It looks like this one should be reused between different effects
        let fft_planner = FftPlanner::new();
        let fft_forward = fft_planner.plan_fft_forward(fft_size);
        let fft_inverse = fft_planner.plan_fft_inverse(fft_size);

        //let num_real_samples = fft_forward.;

        Self {
            num_channels: effect_settings.num_channels,
            impulse_response_size: effect_settings.ir_size,
            frame_size,
            window: Vec::with_capacity(window_size),
            fft_forward,
            fft_inverse,
            windowed_dry: Vec::with_capacity(capacity),
            fft_windowed_dry: (),
            dry: (),
            wet: (),
            fft_wet: (),
            overlap: (),
            num_tail_samples_remaining: (),
        }
    }

    fn apply(
        &mut self,
        parameters: OverlapAddConvolutionEffectParams,
        input: &[&[f32]],
        output: &[&mut [f32]],
    ) {
        // Steam Audio assertions:
        // num samples in == num sample out
        // num channels in == 1 or self.num_channels
        // num channels out == self.num_channels
        // Assuming one channel in for now
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fft() {
        let fft_size = 3;
        let mut fft_planner = FftPlanner::new();
        let fft_forward = fft_planner.plan_fft_forward(fft_size);
        let fft_inverse = fft_planner.plan_fft_inverse(fft_size);

        let mut input: Vec<Complex<f32>> = Vec::default();

        fft_forward.pr
    }
}
