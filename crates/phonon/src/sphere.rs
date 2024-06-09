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

pub(crate) struct Sphere {
    pub(crate) center: Vec3,
    pub(crate) radius: f32,
}

impl Sphere {
    pub(crate) fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }

    pub(crate) fn contains(&self, point: Vec3) -> bool {
        (point - self.center).length_squared() <= self.radius * self.radius
    }
}
