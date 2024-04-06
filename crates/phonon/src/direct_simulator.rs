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

use crate::bands;
use crate::sampling::generate_sphere_volume_sample;
use glam::Vec3;

enum DirectSimulationType {
    CalcDistanceAttenuation,
    CalcAirAbsorption,
    CalcDirectivity,
    CalcOcclusion,
    CalcTransmission,
    CalcDela,
}

enum OcclusionType {
    Raycast,
    Volumetric,
}

/// Describes the properties of a direct sound path.
struct DirectSoundPath {
    distance_attenuation: f32,
    air_absorption: [f32; bands::NUM_BANDS],
    delay: f32,
    occlusion: f32,
    transmission: [f32; bands::NUM_BANDS],
    directivity: f32,
}

/// Encapsulates the state required to simulate direct sound, including distance
/// attenuation, air absorption, partial occlusion, and propagation delays.
struct DirectSimulator {
    /// Sampling points distributed inside a spherical volume.
    ///
    /// The amount of sampling points taken can be configured per
    /// source, up to `max_occlusion_samples`.
    /// These sampling points are transformed to the source position
    /// when calculating the volumetric occlusion.
    sphere_volume_samples: Vec<Vec3>,
}

impl DirectSimulator {
    fn new(max_occlusion_samples: usize) -> Self {
        let mut sphere_volume_samples = Vec::new();

        for i in 0..max_occlusion_samples {
            sphere_volume_samples.push(generate_sphere_volume_sample(i));
        }

        Self {
            sphere_volume_samples,
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
