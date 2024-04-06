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

use glam::Vec3;
use crate::bands;
use crate::sampling::generate_sphere_volume_sample;

enum DirectSimulationType {
    CalcDistanceAttenuation,
    CalcAirAbsorption,
    CalcDirectivity,
    CalcOcclusion,
    CalcTransmission,
    CalcDela,
}

enum OcclusionType {
    Raycast,
    Volumetric,
}

struct DirectSoundPath {
    distance_attenuation: f32,
    air_absorption: [f32; bands::NUM_BANDS],
    delay: f32,
    occlusion: f32,
    transmission: [f32; bands::NUM_BANDS],
    directivity: f32,
}

struct DirectSimulator {
    sphere_volume_samples: Vec<Vec3>,
}

impl DirectSimulator {
    fn new(mut self, max_occlusion_samples: usize) -> Self {
        let mut sphere_volume_samples = Vec::new();

        for i in 0..max_occlusion_samples {
            sphere_volume_samples.push(generate_sphere_volume_sample(i));
        }

        Self {
            sphere_volume_samples
        }
    }
}