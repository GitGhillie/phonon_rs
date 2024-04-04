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

use glam::{Vec2, Vec3, Vec4};
use std::mem::size_of;

#[test]
fn vector_size() {
    assert_eq!(8, size_of::<Vec2>());
    assert_eq!(12, size_of::<Vec3>());
    assert_eq!(16, size_of::<Vec4>());
}

// todo: Check if glam already has all the test cases for Vector
