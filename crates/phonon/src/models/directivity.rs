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

use crate::scene::coordinate_space::CoordinateSpace3f;
use glam::Vec3;

// todo: Describe what these do and what the limits are
pub struct Directivity {
    pub dipole_weight: f32,
    pub dipole_power: f32,
}

impl Default for Directivity {
    fn default() -> Self {
        Self {
            dipole_weight: 0.0,
            dipole_power: 1.0,
        }
    }
}

impl Directivity {
    fn evaluate(&self, direction: Vec3) -> f32 {
        let cosine = -direction.z;
        let base = (1.0 - self.dipole_weight) + self.dipole_weight * cosine;
        base.abs().powf(self.dipole_power)
    }

    pub(crate) fn evaluate_at(&self, point: Vec3, coordinates: &CoordinateSpace3f) -> f32 {
        if self.dipole_weight == 0.0 {
            return 1.0;
        }

        let world_space_direction = (point - coordinates.origin).normalize_or_zero();
        let local_space_direction = coordinates.direction_to_local(world_space_direction);
        self.evaluate(local_space_direction)
    }
}
