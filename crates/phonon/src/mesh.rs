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

use crate::triangle::Triangle;
use glam::{Vec3, Vec4};
use ndarray::Array1;

/// A triangle mesh. Vertices are stored in a contiguous array, and the triangles are stored in indexed form. Each
/// triangle requires three indices to store (i.e., strip or fan representations are not supported).
pub struct Mesh {
    vertices: Array1<Vec4>,
    triangles: Array1<Triangle>,
    normals: Array1<Vec3>,
}

impl Mesh {
    pub fn new(vertices: &[Vec3], triangles: &[Triangle]) -> Self {
        let num_triangles = triangles.len();

        let mut mesh = Self {
            vertices: Array1::from_shape_fn(vertices.len(), |i| vertices[i].extend(1.0)),
            triangles: Array1::from_shape_fn(triangles.len(), |i| triangles[i]),
            normals: Array1::default(num_triangles),
        };

        mesh.calculate_normals();

        mesh
    }

    fn calculate_normals(&mut self) {
        let num_triangles = self.triangles.len();

        for i in 0..num_triangles {
            let v0: Vec3 = self.vertices[self.triangles[i].indices[0]].truncate();
            let v1: Vec3 = self.vertices[self.triangles[i].indices[1]].truncate();
            let v2: Vec3 = self.vertices[self.triangles[i].indices[2]].truncate();

            self.normals[i] = (v1 - v0).cross(v2 - v0).normalize_or_zero();
        }
    }

    pub fn get_normal(&self, index: usize) -> Vec3 {
        self.normals[index]
    }
}
