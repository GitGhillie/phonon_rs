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

use rustfft::Fft;

pub struct OverlapAddConvolutionEffectSettings {
    pub num_channels: u8,
    pub ir_size: u32,
}

pub struct OverlapAddConvolutionEffectParams {
    pub fft_impulse_response: Vec<u8>,
    pub multiple_inputs: bool,
}

pub struct OverlapAddConvolutionEffect {
    num_channels: usize,
    impulse_response_size: usize,
    frame_size: usize,
    window: Vec<f32>,
    fft: Arc<dyn Fft<f32>>,
    // todo
}

impl OverlapAddConvolutionEffect {
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
    }
}
