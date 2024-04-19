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
use crate::ray::Ray;
use crate::scene::Scene;
use glam::Mat4;
use std::cell::RefCell;
use std::rc::Rc;

// Port note: mNumVertices and mNumTriangles have been left out, there doesn't seem to be a use.
pub struct InstancedMesh {
    sub_scene: Rc<RefCell<Scene>>,
    transform: Mat4,
    inverse_transform: Mat4,
    /// Flag indicating whether this instanced mesh has changed since the last call to commit().
    has_changed: bool,
}

impl InstancedMesh {
    // Port note: Original code transposes the transform. For performance reasons maybe?
    pub fn new(sub_scene: Rc<RefCell<Scene>>, transform: Mat4) -> RefCell<Self> {
        RefCell::new(Self {
            sub_scene,
            transform,
            inverse_transform: transform.inverse(),
            has_changed: false,
        })
    }

    pub(crate) fn commit(&mut self) {
        self.sub_scene.borrow_mut().commit();

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
        let hit_maybe =
            self.sub_scene
                .borrow()
                .closest_hit(&transformed_ray, min_distance, max_distance);

        return if let Some(hit) = hit_maybe {
            Some(self.transform_hit(&hit, &transformed_ray))
        } else {
            None
        };
    }

    // todo implement min_distance
    pub(crate) fn any_hit(&self, ray: &Ray, min_distance: f32, max_distance: f32) -> bool {
        let mut min_distance = min_distance;
        let mut max_distance = max_distance;

        let transformed_ray = self.inverse_transform_ray(ray, &mut min_distance, &mut max_distance);

        self.sub_scene
            .borrow()
            .any_hit(&transformed_ray, min_distance, max_distance)
    }

    // todo: use Mat4 transform point fns
    /// Returns a `Ray` transformed back to the original mesh transformation.
    /// `min_distance` and `max_distance` get changed accordingly.
    fn inverse_transform_ray(
        &self,
        ray: &Ray,
        min_distance: &mut f32,
        max_distance: &mut f32,
    ) -> Ray {
        let origin = self.inverse_transform * ray.origin().extend(1.0);

        let start = self.inverse_transform * ray.point_at_distance(*min_distance).extend(1.0);
        *min_distance = (start - origin).length();

        if *max_distance < f32::MAX {
            let end = self.inverse_transform * ray.point_at_distance(*max_distance).extend(1.0);
            *max_distance = (end - origin).length();
        }

        let direction = (self.inverse_transform * ray.direction().extend(0.0))
            .truncate()
            .normalize_or_zero();

        // Return a transformed Ray
        Ray::new(origin.truncate(), direction)
    }

    // todo: use Mat4 transform point fns
    fn transform_hit(&self, hit: &Hit, ray: &Ray) -> Hit {
        let mut transformed_hit = hit.clone();

        let origin = self.transform * ray.origin().extend(1.0);
        let hit_point = self.transform * ray.point_at_distance(hit.distance).extend(1.0);

        transformed_hit.distance = (hit_point - origin).length();
        transformed_hit.normal = (self.transform * hit.normal.extend(0.0))
            .truncate()
            .normalize_or_zero();

        transformed_hit
    }
}
