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

use crate::sphere::Sphere;
use glam::Vec3;
use std::f32::consts::PI;

// https://stackoverflow.com/questions/5408276/sampling-uniformly-distributed-random-points-inside-a-spherical-volume
/// Generate a point inside a spherical volume
pub(crate) fn generate_sphere_volume_sample(i: usize) -> Vec3 {
    let u_phi = radical_inverse(2, i);
    let u_theta = radical_inverse(3, i);
    let u_r = radical_inverse(5, i);

    let phi = 2.0 * PI * u_phi;
    let theta = (2.0 * u_theta - 1.0).acos();
    let r = u_r.cbrt();

    let x = r * theta.sin() * phi.cos();
    let y = r * theta.sin() * phi.sin();
    let z = r * theta.cos();

    Vec3::new(x, y, z)
}

pub(crate) fn transform_sphere_volume_sample(sample: Vec3, sphere: Sphere) -> Vec3 {
    sphere.center + (sample * sphere.radius)
}

// http://www.pbr-book.org/3ed-2018/Sampling_and_Reconstruction/The_Halton_Sampler.html#RadicalInverseSpecialized
fn radical_inverse(p: i32, i: usize) -> f32 {
    let inv = 1.0 / (p as f32);
    let mut reversed = 0;
    let mut inv_n = 1.0f32;

    let mut i_loop = i as i32;
    while i_loop != 0 {
        let next = i_loop / p;
        let digit = i_loop - (next * p);
        reversed = reversed * p + digit;
        inv_n *= inv;
        i_loop = next;
    }

    (reversed as f32 * inv_n).min(1.0)
}
