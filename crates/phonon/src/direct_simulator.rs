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

use crate::air_absorption::AirAbsorptionModel;
use crate::bands::NUM_BANDS;
use crate::coordinate_space::CoordinateSpace3f;
use crate::direct_effect::DirectApplyFlags;
use crate::directivity::Directivity;
use crate::distance_attenuation::DistanceAttenuationModel;
use crate::propagation_medium::SPEED_OF_SOUND;
use crate::sampling::{generate_sphere_volume_sample, transform_sphere_volume_sample};
use crate::scene::Scene;
use crate::sphere::Sphere;
use glam::Vec3;

// todo: Remove in favor of DirectApplyFlags?
enum DirectSimulationType {
    CalcDistanceAttenuation,
    CalcAirAbsorption,
    CalcDirectivity,
    CalcOcclusion,
    CalcTransmission,
    CalcDelay,
}

pub enum OcclusionType {
    Raycast,
    Volumetric,
}

/// Describes the properties of a direct sound path.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectSoundPath {
    pub distance_attenuation: f32,
    pub air_absorption: [f32; NUM_BANDS],
    pub delay: f32,
    pub occlusion: f32,
    pub transmission: [f32; NUM_BANDS],
    pub directivity: f32,
}

impl Default for DirectSoundPath {
    fn default() -> Self {
        Self {
            distance_attenuation: 1.0,
            air_absorption: [1.0, 1.0, 1.0],
            delay: 0.0,
            occlusion: 1.0,
            transmission: [1.0, 1.0, 1.0],
            directivity: 0.0,
        }
    }
}

/// Encapsulates the state required to simulate direct sound, including distance
/// attenuation, air absorption, partial occlusion, and propagation delays.
pub struct DirectSimulator {
    /// Sampling points distributed inside a spherical volume.
    ///
    /// The amount of sampling points taken can be configured per
    /// source, up to `max_occlusion_samples`.
    /// These sampling points are transformed to the source position
    /// when calculating the volumetric occlusion.
    sphere_volume_samples: Vec<Vec3>,
}

impl DirectSimulator {
    pub fn new(max_occlusion_samples: usize) -> Self {
        let mut sphere_volume_samples = Vec::new();

        for i in 0..max_occlusion_samples {
            sphere_volume_samples.push(generate_sphere_volume_sample(i));
        }

        Self {
            sphere_volume_samples,
        }
    }

    pub fn simulate(
        &self,
        scene: &Scene,
        flags: DirectApplyFlags,
        source: CoordinateSpace3f,
        listener: CoordinateSpace3f,
        distance_attenuation_model: &impl DistanceAttenuationModel,
        air_absorption_model: &impl AirAbsorptionModel,
        directivity: Directivity,
        occlusion_type: OcclusionType,
        occlusion_radius: f32,
        num_occlusion_samples: usize,
        num_transmission_rays: i32,
        direct_sound_path: &mut DirectSoundPath,
    ) {
        let distance = (source.origin - listener.origin).length();

        if flags.contains(DirectApplyFlags::DistanceAttenuation) {
            direct_sound_path.distance_attenuation = distance_attenuation_model.evaluate(distance);
        } else {
            direct_sound_path.distance_attenuation = 1.0
        }

        if flags.contains(DirectApplyFlags::AirAbsorption) {
            for i in 0..NUM_BANDS {
                direct_sound_path.air_absorption[i] = air_absorption_model.evaluate(distance, i);
            }
        } else {
            for i in 0..NUM_BANDS {
                direct_sound_path.air_absorption[i] = 1.0;
            }
        }

        if flags.contains(DirectApplyFlags::Delay) {
            direct_sound_path.delay = Self::direct_path_delay(listener.origin, source.origin);
        } else {
            direct_sound_path.delay = 0.0;
        }

        if flags.contains(DirectApplyFlags::Directivity) {
            direct_sound_path.directivity = directivity.evaluate_at(listener.origin, &source);
        } else {
            direct_sound_path.directivity = 1.0;
        }

        // todo: The scene must be optional
        if flags.contains(DirectApplyFlags::Occlusion) {
            match occlusion_type {
                OcclusionType::Raycast => {
                    direct_sound_path.occlusion =
                        Self::raycast_occlusion(scene, listener.origin, source.origin);
                }
                OcclusionType::Volumetric => {
                    direct_sound_path.occlusion = Self::raycast_volumetric(
                        self,
                        scene,
                        listener.origin,
                        source.origin,
                        occlusion_radius,
                        num_occlusion_samples,
                    );
                }
            }
        } else {
            direct_sound_path.occlusion = 1.0
        }

        // todo transmission stuff
    }

    fn direct_path_delay(listener: Vec3, source: Vec3) -> f32 {
        (source - listener).length() / SPEED_OF_SOUND
    }

    fn raycast_occlusion(scene: &Scene, listener_position: Vec3, source_position: Vec3) -> f32 {
        match scene.is_occluded(listener_position, source_position) {
            false => 1.0,
            true => 0.0,
        }
    }

    fn raycast_volumetric(
        &self,
        scene: &Scene,
        listener_position: Vec3,
        source_position: Vec3,
        source_radius: f32,
        num_samples: usize,
    ) -> f32 {
        let mut occlusion: f32 = 0.0;
        let mut num_valid_samples = 0;

        let num_samples = self.sphere_volume_samples.len().min(num_samples);

        for i in 0..num_samples {
            let sphere = Sphere::new(source_position, source_radius);
            let sample = transform_sphere_volume_sample(self.sphere_volume_samples[i], sphere);

            if scene.is_occluded(source_position, sample) {
                continue;
            }

            num_valid_samples += 1;

            if !scene.is_occluded(listener_position, sample) {
                occlusion += 1.0;
            }
        }

        if num_valid_samples == 0 {
            return 0.0;
        }

        occlusion / num_valid_samples as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use plotters::prelude::*;

    const OUT_FILE_NAME: &str = "figures/sphere_volume_samples.gif";

    #[ignore = "visual check only."]
    #[test]
    fn direct_simulator_samples() {
        let num_frames_gif = 30;
        let max_occlusion_samples = 100;
        let num_samples_source = 10; // must be less than `max_occlusion_samples`

        let simulator = DirectSimulator::new(max_occlusion_samples);
        // Pretend the source is using `num_samples_source` samples.
        // The source samples will be drawn in red, the remaining samples in blue.
        let mut points_source = simulator.sphere_volume_samples;
        let points_remaining = points_source.split_off(num_samples_source);

        let drawing_area = BitMapBackend::gif(OUT_FILE_NAME, (1024, 760), 100)
            .unwrap()
            .into_drawing_area();
        let x_axis = (-1.5..1.5).step(0.1);
        let z_axis = (-1.5..1.5).step(0.1);

        for yaw in 0..num_frames_gif {
            drawing_area.fill(&WHITE).unwrap();

            let mut chart = ChartBuilder::on(&drawing_area)
                .caption(
                    "Volumetric Occlusion Sampling Sphere".to_string(),
                    ("sans", 20),
                )
                .build_cartesian_3d(x_axis.clone(), -1.5..1.5, z_axis.clone())
                .unwrap();

            chart.with_projection(|mut pb| {
                pb.yaw = 1.00 - ((num_frames_gif as f64 / 100.0) - yaw as f64 / 50.0).abs();
                pb.scale = 0.9;
                pb.into_matrix()
            });

            chart
                .configure_axes()
                .light_grid_style(BLACK.mix(0.15))
                .max_light_lines(3)
                .draw()
                .unwrap();

            chart
                .draw_series(points_source.iter().map(|point| {
                    Circle::new(
                        (point.x as f64, point.y as f64, point.z as f64),
                        2,
                        RED.filled(),
                    )
                }))
                .unwrap();

            chart
                .draw_series(points_remaining.iter().map(|point| {
                    Circle::new(
                        (point.x as f64, point.y as f64, point.z as f64),
                        2,
                        BLUE.filled(),
                    )
                }))
                .unwrap();

            drawing_area.present().unwrap();
        }

        // To avoid the IO failure being ignored silently, manually call the present function
        drawing_area.present().expect(
            "Unable to write result to file, please make sure 'figures' dir exists in crate dir",
        );
        println!("Result has been saved to {}", OUT_FILE_NAME);
    }
}
