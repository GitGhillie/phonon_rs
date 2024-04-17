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

use crate::ray::Ray;
use crate::scene::Scene;
use glam::Mat4;
use std::rc::Rc;

// Port note: mNumVertices and mNumTriangles have been left out, there doesn't seem to be a use.
pub struct InstancedMesh {
    sub_scene: Rc<Scene>,
    transform: Mat4,
    inverse_transform: Mat4,
    /// Flag indicating whether this instanced mesh has changed since the last call to commit().
    has_changed: bool,
}

impl InstancedMesh {
    fn new(sub_scene: Rc<Scene>, transform: Mat4) -> Self {
        // todo: Original code transposes the transform. Check if necessary.
        Self {
            sub_scene,
            transform,
            inverse_transform: transform.inverse(),
            has_changed: false,
        }
    }

    // todo implement min_distance
    fn any_hit(&self, ray: Ray, min_distance: f32, max_distance: f32) -> bool {
        let mut min_distance = min_distance;
        let mut max_distance = max_distance;

        let transformed_ray = self.inverse_transform_ray(ray, &mut min_distance, &mut max_distance);

        //self.sub_scene.any_hit //todo
        false //todo remove
    }

    fn inverse_transform_ray(
        &self,
        ray: Ray,
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

        let p = self.inverse_transform * ray.point_at_distance(1.0).extend(1.0);
        let direction = (p.truncate() - origin.truncate()).normalize_or_zero();

        // Return the transformed ray
        Ray::new(origin.truncate(), direction)
    }
}
