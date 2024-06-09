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

use crate::hit::Hit;
use crate::material::Material;
use crate::mesh::Mesh;
use crate::ray::Ray;
use crate::triangle::Triangle;
use glam::Vec3;
use ndarray::Array1;
use parry3d::query::RayCast;

/// A static triangle mesh. The geometry of this mesh is assumed to never change at runtime. It is described in
/// world-space coordinates. Materials are specified for each triangle.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StaticMesh {
    mesh: Mesh,
    material_indices: Array1<usize>,
    materials: Array1<Material>,
}

/// An IStaticMesh implementation that uses the built-in ray tracer backend.
impl StaticMesh {
    pub fn new(
        vertices: Vec<Vec3>,
        triangles: Vec<Triangle>,
        material_indices: Vec<usize>,
        materials: Vec<Material>,
    ) -> Self {
        Self {
            mesh: Mesh::new(vertices, triangles),
            material_indices: material_indices.into(),
            materials: materials.into(),
        }
    }

    // todo ability to create mesh with a single material
    pub fn new_from_mesh(
        mesh: Mesh,
        //material_indices: Vec<usize>,
        //materials: Vec<Material>,
    ) -> Self {
        let material = Material {
            absorption: [0.5, 0.3, 0.1],
            scattering: 0.05,
            transmission: [0.5, 0.3, 0.1],
        };

        let num_triangles = mesh.mesh.num_triangles();
        let materials = vec![material];

        Self {
            mesh,
            material_indices: Array1::zeros(num_triangles),
            materials: materials.into(),
        }
    }

    pub(crate) fn closest_hit(
        &self,
        ray: &Ray,
        min_distance: f32,
        max_distance: f32,
    ) -> Option<Hit> {
        let ray = Ray::new(ray.point_at_distance(min_distance), ray.direction());

        // todo: What does `solid` need to be?
        let info = self
            .mesh
            .mesh
            .cast_local_ray_and_get_normal(&ray.0, max_distance, false);

        return if let Some(hit) = info {
            let mut triangle_index = hit.feature.unwrap_face() as usize;

            if triangle_index >= self.mesh.mesh.indices().len() {
                // A backface is hit, which means the index needs to be put back into bounds.
                // See source code of `cast_local_ray_and_get_normal`.
                triangle_index -= self.mesh.mesh.indices().len();
            }

            let material_index = self.material_indices[triangle_index];

            Some(Hit {
                distance: hit.toi,
                triangle_index,
                object_index: 0,
                material_index,
                normal: hit.normal.into(),
                material: self.materials[material_index],
            })
        } else {
            None
        };
    }

    pub(crate) fn any_hit(&self, ray: &Ray, min_distance: f32, max_distance: f32) -> bool {
        let ray = Ray::new(ray.point_at_distance(min_distance), ray.direction());

        self.mesh
            .mesh
            .cast_local_ray(&ray.0, max_distance, false)
            .is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_mesh() {
        let vertices = vec![
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(2.0, 0.0, 0.0),
        ];

        let triangle0 = Triangle { indices: [0, 1, 2] };
        let triangle1 = Triangle { indices: [1, 2, 3] };
        let triangle2 = Triangle { indices: [1, 3, 4] };
        let triangles = vec![triangle0, triangle1, triangle2];

        let material = Material {
            absorption: [0.1, 0.1, 0.1],
            scattering: 0.05,
            transmission: [0.0, 0.0, 0.0],
        };

        let materials = vec![material];
        let material_indices = vec![0, 0, 0];

        let static_mesh = StaticMesh::new(vertices, triangles, material_indices, materials);

        let ray0: Ray = Ray::new(Vec3::new(0.1, 0.1, -1.0), Vec3::new(0.0, 0.0, 1.0));
        let ray1: Ray = Ray::new(Vec3::new(0.6, 0.6, -1.0), Vec3::new(0.0, 0.0, 1.0));
        let ray2: Ray = Ray::new(Vec3::new(1.1, 0.1, -1.0), Vec3::new(0.0, 0.0, 1.0));

        let ray_miss: Ray = Ray::new(Vec3::new(1.0, 0.0, -1.0), Vec3::new(0.0, 1.0, 0.0));

        let hit0 = static_mesh.closest_hit(&ray0, 0.0, 10.0);
        let hit1 = static_mesh.closest_hit(&ray1, 0.0, 10.0);
        let hit2 = static_mesh.closest_hit(&ray2, 0.0, 10.0);

        assert_eq!(hit0.unwrap().triangle_index, 0);
        assert_eq!(hit1.unwrap().triangle_index, 1);
        assert_eq!(hit2.unwrap().triangle_index, 2);

        assert!(!static_mesh.any_hit(&ray_miss, 0.0, 10.0));
    }
}
