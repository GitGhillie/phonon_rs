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
use parry3d;

pub(crate) struct Ray(pub(crate) parry3d::query::Ray);

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self(parry3d::query::Ray::new(origin.into(), direction.into()))
    }

    pub(crate) fn origin(&self) -> Vec3 {
        self.0.origin.into()
    }

    pub(crate) fn direction(&self) -> Vec3 {
        self.0.dir.into()
    }

    pub(crate) fn point_at_distance(&self, distance: f32) -> Vec3 {
        self.origin() + (distance * self.direction())
    }
}
