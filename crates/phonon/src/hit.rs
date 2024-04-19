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

use crate::material::Material;
use glam::Vec3;

#[derive(Copy, Clone, Debug)]
pub(crate) struct Hit {
    pub(crate) distance: f32,
    pub(crate) triangle_index: usize,
    pub(crate) object_index: usize,
    pub(crate) material_index: usize,
    pub(crate) normal: Vec3,
    pub(crate) material: Material,
}

impl Hit {
    pub(crate) fn new_empty() -> Self {
        Self {
            distance: f32::MAX,
            triangle_index: 0,
            object_index: 0,
            material_index: 0,
            normal: Default::default(),
            //todo better Material default
            material: Material {
                absorption: [0.0, 0.0, 0.0],
                scattering: 0.0,
                transmission: [0.0, 0.0, 0.0],
            },
        }
    }
}
