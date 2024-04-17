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
use crate::static_mesh::StaticMesh;
use std::rc::Rc;

/// A 3D scene, comprised of multiple kinds of SceneObjects. Objects can be added and removed from the scene at any
/// time. Objects can also be defined as instances of one another. This class also allows rays to be traced through
/// the scene.
pub struct Scene {
    //todo: Explain why there are two vectors of each
    pub(crate) static_meshes: [Vec<Rc<StaticMesh>>; 2],
    pub(crate) instanced_meshes: [Vec<Rc<InstancedMesh>>; 2],
    /// Flag indicating whether the scene has changed in some way since the previous call to commit().
    has_changed: bool,
    /// The change version of the scene.
    change_version: u32,
}
