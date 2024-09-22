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

use crate::scene::hit::Hit;
use crate::scene::ray::Ray;
use crate::scene::Scene;
use glam::Mat4;
use std::sync::{Arc, Mutex};

// Port note: mNumVertices and mNumTriangles have been left out, there doesn't seem to be a use.
/// A triangle mesh that can be moved (translated), rotated, or scaled, but cannot deform.
///
/// Portions of a scene that undergo rigid-body motion can be represented as instanced meshes.
/// An instanced mesh is essentially a scene (called the “sub-scene”) with a transform applied to it.
/// Adding an instanced mesh to a scene places the sub-scene into the scene with the transform applied.
/// For example, the sub-scene may be a prefab door, and the transform can be used to place it
/// in a doorway and animate it as it opens or closes.
pub struct InstancedMesh {
    sub_scene: Arc<Mutex<Scene>>,
    transform: Mat4,
    inverse_transform: Mat4,
    /// Flag indicating whether this instanced mesh has changed since the last call to commit().
    has_changed: bool,
}

impl InstancedMesh {
    // Port note: Original code transposes the transform. For performance reasons maybe?
    pub fn new(sub_scene: Arc<Mutex<Scene>>, transform: Mat4) -> Mutex<Self> {
        Mutex::new(Self {
            sub_scene,
            transform,
            inverse_transform: transform.inverse(),
            has_changed: false,
        })
    }

    pub fn set_transform(&mut self, transform: Mat4) {
        self.transform = transform;
        self.inverse_transform = transform.inverse();
        //self.has_changed = true; // if something is broken uncomment this (todo remove me)
    }

    pub(crate) fn commit(&mut self) {
        self.sub_scene.lock().unwrap().commit();

        // After calling commit(), this instanced mesh will be considered unchanged until a subsequent call to
        // updateTransform() changes the transform matrix.
        self.has_changed = false;
    }

    pub(crate) fn has_changed(&self) -> bool {
        self.has_changed
    }

    pub(crate) fn closest_hit(
        &self,
        ray: &Ray,
        min_distance: f32,
        max_distance: f32,
    ) -> Option<Hit> {
        let mut min_distance = min_distance;
        let mut max_distance = max_distance;

        let transformed_ray = self.inverse_transform_ray(ray, &mut min_distance, &mut max_distance);
        let hit_maybe = self.sub_scene.lock().unwrap().closest_hit(
            &transformed_ray,
            min_distance,
            max_distance,
        );

        return if let Some(hit) = hit_maybe {
            Some(self.transform_hit(&hit, &transformed_ray))
        } else {
            None
        };
    }

    pub(crate) fn any_hit(&self, ray: &Ray, min_distance: f32, max_distance: f32) -> bool {
        let mut min_distance = min_distance;
        let mut max_distance = max_distance;

        let transformed_ray = self.inverse_transform_ray(ray, &mut min_distance, &mut max_distance);

        self.sub_scene
            .lock()
            .unwrap()
            .any_hit(&transformed_ray, min_distance, max_distance)
    }

    /// Returns a `Ray` transformed back to the original mesh transformation.
    /// `min_distance` and `max_distance` get changed accordingly.
    fn inverse_transform_ray(
        &self,
        ray: &Ray,
        min_distance: &mut f32,
        max_distance: &mut f32,
    ) -> Ray {
        let origin = self.inverse_transform.transform_point3(ray.origin());

        let start = self
            .inverse_transform
            .transform_point3(ray.point_at_distance(*min_distance));
        *min_distance = (start - origin).length();

        if *max_distance < f32::MAX {
            let end = self
                .inverse_transform
                .transform_point3(ray.point_at_distance(*max_distance));
            *max_distance = (end - origin).length();
        }

        let direction = self
            .inverse_transform
            .transform_vector3(ray.direction())
            .normalize_or_zero();

        // Return a transformed Ray
        Ray::new(origin, direction)
    }

    fn transform_hit(&self, hit: &Hit, ray: &Ray) -> Hit {
        let mut transformed_hit = hit.clone();

        let origin = self.transform.transform_point3(ray.origin());
        let hit_point = self
            .transform
            .transform_point3(ray.point_at_distance(hit.distance));

        transformed_hit.distance = (hit_point - origin).length();
        transformed_hit.normal = self
            .transform
            .transform_vector3(hit.normal)
            .normalize_or_zero();

        transformed_hit
    }
}
