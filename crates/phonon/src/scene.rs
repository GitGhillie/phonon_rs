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

use crate::instanced_mesh::InstancedMesh;
use crate::ray::Ray;
use crate::static_mesh::StaticMesh;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

/// A 3D scene, comprised of multiple kinds of SceneObjects. Objects can be added and removed from the scene at any
/// time. Objects can also be defined as instances of one another. This class also allows rays to be traced through
/// the scene.
pub struct Scene {
    //todo: Explain why there are two vectors of each
    //todo: Take a better look if Rc<RefCell<>> is the smart thing to do here.
    pub(crate) static_meshes: [Vec<Rc<RefCell<StaticMesh>>>; 2],
    pub(crate) instanced_meshes: [Vec<Rc<RefCell<InstancedMesh>>>; 2],
    /// Flag indicating whether the scene has changed in some way since the previous call to commit().
    has_changed: bool,
    /// The change version of the scene.
    change_version: u32,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            static_meshes: [Vec::default(), Vec::default()],
            instanced_meshes: [Vec::default(), Vec::default()],
            has_changed: false,
            change_version: 0,
        }
    }

    pub fn add_static_mesh(&mut self, static_mesh: Rc<RefCell<StaticMesh>>) {
        self.static_meshes[1].push(static_mesh);
        self.has_changed = true;
    }

    pub fn remove_static_mesh(&mut self, static_mesh: Rc<RefCell<StaticMesh>>) {
        self.static_meshes[1].retain(|x| x.as_ptr() != static_mesh.as_ptr());
    }

    // todo copy docs on commit and other functions
    pub fn commit(&mut self) {
        // If no static/instanced meshes have been added or removed since the last commit(), check to see if any
        // instanced meshes have had their transforms updated.
        if !self.has_changed {
            for instanced_mesh in &self.instanced_meshes[0] {
                if instanced_mesh.borrow().has_changed() {
                    self.has_changed = true;
                    break;
                }
            }
        }

        // If something changed in the scene, increment the version.
        if self.has_changed {
            self.change_version += 1;
        }

        self.static_meshes[0] = self.static_meshes[1].clone();
        self.instanced_meshes[0] = self.instanced_meshes[1].clone();

        for instanced_mesh in &self.instanced_meshes[0] {
            instanced_mesh.borrow_mut().commit();
        }

        // The scene will be considered unchanged until something is changed subsequently.
        self.has_changed = false;
    }

    pub(crate) fn any_hit(&self, ray: &Ray, min_distance: f32, max_distance: f32) -> bool {
        for static_mesh in &self.static_meshes[0] {
            if static_mesh
                .borrow()
                .any_hit(ray, min_distance, max_distance)
            {
                return true;
            }
        }

        for instanced_mesh in &self.instanced_meshes[0] {
            if instanced_mesh
                .borrow()
                .any_hit(ray, min_distance, max_distance)
            {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::Material;
    use crate::triangle::Triangle;
    use glam::Vec3;

    #[test]
    fn test_scene() {
        let vertices = vec![
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ];

        let triangle0 = Triangle { indices: [0, 1, 2] };

        let triangles = vec![triangle0];

        let material = Material {
            absorption: [0.1, 0.1, 0.1],
            scattering: 0.05,
            transmission: [0.0, 0.0, 0.0],
        };

        let materials = vec![material];
        let material_indices = vec![0];

        let static_mesh = Rc::new(StaticMesh::new(
            vertices,
            triangles,
            material_indices,
            materials,
        ));

        let mut scene = Scene::new();

        scene.add_static_mesh(static_mesh.clone());
        scene.commit();

        let ray_hit: Ray = Ray::new(Vec3::new(0.1, 0.1, -1.0), Vec3::new(0.0, 0.0, 1.0));
        let ray_miss: Ray = Ray::new(Vec3::new(1.0, 1.0, -1.0), Vec3::new(0.0, 0.0, 1.0));

        assert!(scene.any_hit(&ray_hit, 0.0, 1.0));
        assert!(!scene.any_hit(&ray_miss, 0.0, 1.0));

        scene.remove_static_mesh(static_mesh);
        scene.commit();

        assert!(!scene.any_hit(&ray_hit, 0.0, 1.0));
    }
}
