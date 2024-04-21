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

use crate::bands::NUM_BANDS;

/// An acoustic material. The acoustic surface properties of an object are represented using multi-band absorption
/// and transmission loss coefficients, and a single random-incidence scattering coefficient.
/// All values are in the 0.0 to 1.0 range.
#[derive(Copy, Clone, Debug)]
pub struct Material {
    pub absorption: [f32; NUM_BANDS],
    pub scattering: f32,
    pub transmission: [f32; NUM_BANDS],
}
