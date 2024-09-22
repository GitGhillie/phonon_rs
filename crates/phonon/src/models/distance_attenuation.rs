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

pub trait DistanceAttenuationModel {
    fn evaluate(&self, distance: f32) -> f32;
}

pub struct DefaultDistanceAttenuationModel {
    pub min_distance: f32,
}

impl Default for DefaultDistanceAttenuationModel {
    fn default() -> Self {
        Self { min_distance: 1.0 }
    }
}

impl DistanceAttenuationModel for DefaultDistanceAttenuationModel {
    fn evaluate(&self, distance: f32) -> f32 {
        1.0 / self.min_distance.max(distance)
    }
}
