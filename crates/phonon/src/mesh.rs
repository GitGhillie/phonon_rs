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
use glam::Vec3;
use ndarray::Array1;
use parry3d::math::Point;
use parry3d::shape::TriMesh;

/// A triangle mesh. Vertices are stored in a contiguous array, and the triangles are stored in indexed form. Each
/// triangle requires three indices to store (i.e., strip or fan representations are not supported).
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Mesh {
    pub(crate) mesh: TriMesh,
    normals: Array1<Vec3>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vec3>, triangles: Vec<Triangle>) -> Self {
        let num_triangles = triangles.len();

        let parry_vertices: Vec<Point<f32>> =
            Vec::from_iter(vertices.iter().map(|vertex| (*vertex).into()));
        let parry_indices = Vec::from_iter(triangles.iter().map(|triangle| {
            [
                triangle.indices[0] as u32,
                triangle.indices[1] as u32,
                triangle.indices[2] as u32,
            ]
        }));

        let parry_mesh = TriMesh::new(parry_vertices, parry_indices);

        let mut mesh = Self {
            mesh: parry_mesh,
            normals: Array1::default(num_triangles),
        };

        mesh.calculate_normals();

        mesh
    }

    // todo better name or From impl
    pub fn new_from_parry(shape: impl Into<TriMesh>) -> Self {
        let parry_mesh: TriMesh = shape.into();
        let num_triangles = parry_mesh.num_triangles();

        let mut mesh = Self {
            mesh: parry_mesh,
            normals: Array1::default(num_triangles),
        };

        mesh.calculate_normals();

        mesh
    }

    fn calculate_normals(&mut self) {
        let triangles = self.mesh.triangles();

        for (i, triangle) in triangles.enumerate() {
            let v0: Vec3 = triangle.vertices()[0].into();
            let v1: Vec3 = triangle.vertices()[1].into();
            let v2: Vec3 = triangle.vertices()[2].into();

            self.normals[i] = (v1 - v0).cross(v2 - v0).normalize_or_zero();
        }
    }

    pub fn get_normal(&self, index: usize) -> Vec3 {
        self.normals[index]
    }
}
