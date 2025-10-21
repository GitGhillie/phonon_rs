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

use crate::dsp::bands::NUM_BANDS;
use crate::effects::direct::DirectApplyFlags;
use crate::models::air_absorption::AirAbsorptionModel;
use crate::models::directivity::Directivity;
use crate::models::distance_attenuation::DistanceAttenuationModel;
use crate::models::propagation_medium::SPEED_OF_SOUND;
use crate::scene::Scene;
use crate::scene::coordinate_space::CoordinateSpace3f;
use crate::scene::ray::Ray;
use crate::scene::sampling::{generate_sphere_volume_sample, transform_sphere_volume_sample};
use crate::scene::sphere::Sphere;
use glam::Vec3;

#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
#[derive(Debug, Clone, Copy)]
pub enum OcclusionType {
    Raycast,
    Volumetric,
}

/// Describes the properties of a direct sound path.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "firewheel",
    derive(firewheel::diff::Diff, firewheel::diff::Patch)
)]
#[repr(C)]
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
            transmission: [0.1, 0.1, 0.1],
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
        scene: Option<&Scene>,
        flags: DirectApplyFlags,
        source: &CoordinateSpace3f,
        listener: &CoordinateSpace3f,
        distance_attenuation_model: &impl DistanceAttenuationModel,
        air_absorption_model: &impl AirAbsorptionModel,
        directivity: Directivity,
        occlusion_type: OcclusionType,
        occlusion_radius: f32,
        num_occlusion_samples: usize,
        num_transmission_rays: usize,
        direct_sound_path: &mut DirectSoundPath,
    ) {
        let distance = (source.origin - listener.origin).length();

        if flags.distance_attenuation {
            direct_sound_path.distance_attenuation = distance_attenuation_model.evaluate(distance);
        } else {
            direct_sound_path.distance_attenuation = 1.0
        }

        if flags.air_absorption {
            for i in 0..NUM_BANDS {
                direct_sound_path.air_absorption[i] = air_absorption_model.evaluate(distance, i);
            }
        } else {
            direct_sound_path.air_absorption.fill(1.0);
        }

        if flags.delay {
            direct_sound_path.delay = Self::direct_path_delay(listener.origin, source.origin);
        } else {
            direct_sound_path.delay = 0.0;
        }

        if flags.directivity {
            direct_sound_path.directivity = directivity.evaluate_at(listener.origin, source);
        } else {
            direct_sound_path.directivity = 1.0;
        }

        if let Some(scene) = scene {
            if flags.occlusion {
                match occlusion_type {
                    OcclusionType::Raycast => {
                        direct_sound_path.occlusion =
                            Self::raycast_occlusion(scene, listener.origin, source.origin);
                    }
                    OcclusionType::Volumetric => {
                        direct_sound_path.occlusion = self.raycast_volumetric(
                            scene,
                            listener.origin,
                            source.origin,
                            occlusion_radius,
                            num_occlusion_samples,
                        );
                    }
                }
            }

            if flags.transmission {
                self.transmission(
                    scene,
                    listener.origin,
                    source.origin,
                    &mut direct_sound_path.transmission,
                    num_transmission_rays,
                );
            }
        } else {
            direct_sound_path.occlusion = 1.0;
            direct_sound_path.transmission.fill(1.0);
        }
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

    /// Each source has a radius, and several points are sampled within the volume
    /// of this sphere. To calculate a source's volumetric occlusion factor, we first
    /// count the number of samples that are visible to the source. (If the source is
    /// close to a wall or the floor, some samples may stick out through the surface,
    /// and these should not be counted when calculating occlusion in the next step.
    /// Essentially the source is shaped like a subset of the sphere's volume, where
    /// the subset is determined by the volumetric samples that do not cross surface
    /// boundaries.) For each sample that's visible to the source, we check whether
    /// it's also visible to the listener. The fraction of samples visible to the
    /// source that are also visible to the listener is then the occlusion factor.
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

    fn transmission(
        &self,
        scene: &Scene,
        listener_position: Vec3,
        source_position: Vec3,
        transmission_factors: &mut [f32],
        num_transmission_rays: usize,
    ) {
        // todo: Warn instead?
        if num_transmission_rays == 0 {
            return;
        }

        // If, after finding a hit point, we want to continue tracing the ray towards the
        // source, then offset the ray origin by this distance along the ray direction, to
        // prevent self-intersection.
        let ray_offset = 1e-2f32;

        // todo: I'm not sure I understand the following. Might be worth investigating.
        // We will alternate between tracing a ray from the listener to the source, and from the source to the listener.
        // The motivation is that if the listener observes the source go behind an object, then that object's material is
        // most relevant in terms of the expected amount of transmitted sound, even if there are multiple other occluders
        // between the source and the listener.
        let rays = [
            Ray::new(
                listener_position,
                (source_position - listener_position).normalize(),
            ),
            Ray::new(
                source_position,
                (listener_position - source_position).normalize(),
            ),
        ];

        let mut current_ray_index = 0;
        let mut hit_count = 0;
        let mut min_distances: [f32; 2] = [0.0, 0.0];
        let max_distance = (source_position - listener_position).length();

        // Product of the transmission coefficients of all hit points.
        let mut accumulated_transmission: [f32; NUM_BANDS] = [1.0, 1.0, 1.0];

        for _ in 0..num_transmission_rays {
            // Select the ray we want to trace for this iteration.
            let ray = &rays[current_ray_index];
            let min_distance = &mut min_distances[current_ray_index];

            let hit = scene.closest_hit(ray, *min_distance, max_distance);

            // If there's nothing more between the ray origin and the source, stop.
            if hit.is_none() {
                break;
            }

            let hit = hit.unwrap();
            hit_count += 1;

            // Accumulate the product of the transmission coefficients of all materials
            // encountered so far.
            for j in 0..NUM_BANDS {
                accumulated_transmission[j] *= hit.material.transmission[j];
            }

            // Calculate the origin of the next ray segment we'll trace, if any.
            *min_distance = hit.distance + ray_offset;
            if *min_distance >= max_distance {
                break;
            }

            // If the total distance traveled by both rays is greater than the distance between
            // the source and the listener, then the rays have crossed, so stop.
            if (min_distances[0] + min_distances[1]) >= max_distance {
                break;
            }

            // Switch to the other ray for the next iteration.
            current_ray_index = 1 - current_ray_index;
        }

        if hit_count <= 1 {
            // If we have only 1 hit, then use the transmission coefficients of that material.
            // If we have no hits, this will automatically set the transmission coefficients to
            // [1.0, 1.0, 1.0] (i.e., 100% transmission).
            transmission_factors.copy_from_slice(accumulated_transmission.as_slice());
        } else {
            // We have more than one hit, so set the total transmission to the square root of the
            // product of the transmission coefficients of all hit points. This assumes that hit
            // points occur in pairs, e.g. both sides of a solid wall, in which case we avoid
            // double-counting the transmission due to both sides of the wall.
            for i in 0..NUM_BANDS {
                transmission_factors[i] = accumulated_transmission[i].sqrt();
            }
        }
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
