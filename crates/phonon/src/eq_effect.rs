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

use ndarray::Array1;
use crate::audio_buffer::AudioSettings;

use crate::bands::NUM_BANDS;
use crate::iir::IIRFilterer;

struct EqEffectParameters {
    gains: Vec<f32>,
}

struct EqEffect {
    sampling_rate: i32,
    frame_size: usize,
    filters: [[IIRFilterer; NUM_BANDS]; 2],
    //todo: document:
    temp: Array1<f32>,
    previous_gains: [f32; NUM_BANDS],
    //todo: document:
    current: i32,
}

impl EqEffect {
    fn new(audio_settings: AudioSettings) -> Self {
        Self {
            sampling_rate: audio_settings.sampling_rate,
            frame_size: audio_settings.frame_size,
            filters: [],
            temp: Default::default(), //todo
            previous_gains: [1.0, 1.0, 1.0],
            current: 0,
        }
    }

    fn set_filter_gains(self, index: i32, gains: &[f32]) {
        let filter = self.filters[0][index];
    }
}