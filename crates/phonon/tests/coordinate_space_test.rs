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
use phonon::coordinate_space::CoordinateSpace3f;

#[test]
fn coordinate_system_right_handed() {
    let test_space = CoordinateSpace3f::from_vectors(Vec3::NEG_Z, Vec3::Y, Vec3::ZERO);

    assert_eq!(Vec3::X, test_space.right);
}

#[test]
fn coordinate_system_orthonormal() {
    let test_space = CoordinateSpace3f::from_vector(Vec3::NEG_X, Vec3::ZERO);

    assert_eq!(0.0, test_space.ahead.dot(test_space.right));
    assert_eq!(0.0, test_space.ahead.dot(test_space.up));
    assert_eq!(0.0, test_space.up.dot(test_space.right));
}

#[test]
fn coordinate_system_transforms() {
    let test_vec = Vec3::new(-3.0, 5.0, 6.0).normalize();
    let test_space = CoordinateSpace3f::from_vector(test_vec, Vec3::ZERO);
    let transformed_ahead = test_space.direction_to_world(&Vec3::NEG_Z);
    assert_eq!(transformed_ahead, test_vec);

    let test_space_x = CoordinateSpace3f::from_vector(Vec3::X, Vec3::ZERO);
    let transformed_x = test_space_x.direction_to_world(&Vec3::NEG_Z);
    assert_eq!(transformed_x, Vec3::X);

    let test_space_y = CoordinateSpace3f::from_vector(Vec3::Y, Vec3::ZERO);
    let transformed_y = test_space_y.direction_to_world(&Vec3::NEG_Z);
    assert_eq!(transformed_y, Vec3::Y);

    let test_space_z = CoordinateSpace3f::from_vector(Vec3::Z, Vec3::ZERO);
    let transformed_z = test_space_z.direction_to_world(&Vec3::NEG_Z);
    assert_eq!(transformed_z, Vec3::Z);
}
