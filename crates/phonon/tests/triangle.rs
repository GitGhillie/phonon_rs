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
use phonon::mesh::Mesh;
use phonon::triangle::Triangle;

#[test]
fn triangle_normals() {
    let vertices = [
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    ];

    let triangle = Triangle { indices: [0, 1, 2] };

    let mesh = Mesh::new(vertices.as_slice(), &[triangle]);

    let normal = mesh.get_normal(0);

    assert_eq!(normal.x, 0.0);
    assert_eq!(normal.y, 0.0);
    assert_eq!(normal.z, 1.0);
}