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

/// Template class that represents a Cartesian coordinate system in 3D, with coordinate axes and origin. The
/// coordinate system is right-handed.
// todo: Actually make it a template class? When needed.
pub struct CoordinateSpace3f {
    pub right: Vec3,  // Unit vector pointing to the right of the origin, i.e., local +x.
    pub up: Vec3,     // Unit vector pointing upwards from the origin, i.e., local +y.
    pub ahead: Vec3,  // Unit vector pointing ahead from the origin, i.e., local -z.
    pub origin: Vec3, // Origin of the coordinate space.
}

impl Default for CoordinateSpace3f {
    /// Constructs the canonical coordinate space. The origin is at the world-space origin, right is along +x,
    /// up is along +y, and ahead is along -z.
    fn default() -> Self {
        CoordinateSpace3f {
            right: Vec3::X,
            up: Vec3::Y,
            ahead: Vec3::NEG_Z,
            origin: Vec3::ZERO,
        }
    }
}

impl CoordinateSpace3f {
    /// Constructs the canonical coordinate space. The origin specified as an argument, right is along +x,
    /// up is along +y, and ahead is along -z.
    pub fn from_origin(origin: Vec3) -> Self {
        CoordinateSpace3f {
            right: Vec3::X,
            up: Vec3::Y,
            ahead: Vec3::NEG_Z,
            origin,
        }
    }

    /// Constructs a coordinate space given two mutually perpendicular vectors (ahead and up), that uniquely
    /// define a right-handed coordinate system.
    pub fn from_vectors(ahead: Vec3, up: Vec3, origin: Vec3) -> Self {
        CoordinateSpace3f {
            right: ahead.cross(up),
            up,
            ahead,
            origin,
        }
    }

    /// Constructs a coordinate space given one vector. A single vector does not uniquely define a coordinate
    /// system. Heuristics are used to select one of the infinitely many possible coordinate systems that have the
    /// ahead vector as one of the axes.
    ///
    /// This algorithm is based on the following paper:
    ///
    ///  Building an orthonormal basis from a unit vector
    ///  J. F. Hughes, T. Moller
    ///  Journal of Graphics Tools 4(4), 1999
    ///  https://pdfs.semanticscholar.org/237c/66be3fe264a11f80f9ad3d2b9ac460e76edc.pdf
    pub fn from_vector(ahead: Vec3, origin: Vec3) -> Self {
        let right: Vec3 = if ahead.x.abs() > ahead.z.abs() {
            Vec3::new(-ahead.y, ahead.x, 0.0).normalize()
        } else {
            Vec3::new(0.0, -ahead.z, ahead.y).normalize()
        };

        let up = right.cross(ahead);

        CoordinateSpace3f {
            right,
            up,
            ahead,
            origin,
        }
    }

    /// Transforms a direction from world space (the canonical coordinate space) to this coordinate space.
    pub fn direction_to_local(&self, direction: &Vec3) -> Vec3 {
        Vec3 {
            x: direction.dot(self.right),
            y: direction.dot(self.up),
            z: -direction.dot(self.ahead),
        }
    }

    /// Transforms a direction from this coordinate space to world space.
    pub fn direction_to_world(&self, direction: &Vec3) -> Vec3 {
        self.right * direction.x + self.up * direction.y - self.ahead * direction.z
    }
}
