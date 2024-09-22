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

use crate::dsp::bands::NUM_BANDS;

pub trait AirAbsorptionModel {
    fn evaluate(&self, distance: f32, band: usize) -> f32;
}

pub struct DefaultAirAbsorptionModel {
    coefficients: [f32; NUM_BANDS],
}

impl Default for DefaultAirAbsorptionModel {
    fn default() -> Self {
        Self {
            coefficients: [0.0002, 0.0017, 0.182],
        }
    }
}

impl AirAbsorptionModel for DefaultAirAbsorptionModel {
    fn evaluate(&self, distance: f32, band: usize) -> f32 {
        (-self.coefficients[band] * distance).exp()
    }
}
