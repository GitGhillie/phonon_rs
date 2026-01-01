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

/// Sound sources can emit sound with different intensities in different
/// directions. For example, a megaphone mostly projects sound towards the
/// front. Steam Audio models this using a directivity pattern. Due to a
/// source’s directivity pattern, and its orientation and position relative
/// to the listener, a further attenuation is applied to it, on top of any
/// distance attenuation or air absorption.
///
/// Steam Audio’s default directivity pattern is a weighted dipole.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Directivity {
    /// Specifies a blend between a monopole (a source that
    /// emits sound equally in all directions) and a dipole (a source that emits
    /// sound mostly to the front and the back). A dipole_weight
    /// value of 0.5f results in a 50% monopole and 50% dipole blend. This is also
    /// called a cardioid directivity pattern.
    pub dipole_weight: f32,
    /// Controls the sharpness of the dipole pattern.
    /// Higher values result in more focused sound.
    ///
    /// Usually between 1.0 and 4.0
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

    /// Evaluate the directivity when the listener is at `point`. Normally this is
    /// called by the simulator.
    pub fn evaluate_at(&self, point: Vec3, coordinates: &CoordinateSpace3f) -> f32 {
        if self.dipole_weight == 0.0 {
            return 1.0;
        }

        let world_space_direction = (point - coordinates.origin).normalize_or_zero();
        let local_space_direction = coordinates.direction_to_local(world_space_direction);
        self.evaluate(local_space_direction)
    }
}
