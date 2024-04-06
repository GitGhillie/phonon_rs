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

    const OUT_FILE_NAME: &str = "figures/sampling-plot.svg";

    #[test]
    fn direct_simulator_samples() {
        let drawing_area = SVGBackend::new(OUT_FILE_NAME, (1024, 760)).into_drawing_area();
        drawing_area.fill(&WHITE)?;

        let x_axis = (-3.0..3.0).step(0.1);
        let z_axis = (-3.0..3.0).step(0.1);

        let mut chart = ChartBuilder::on(&drawing_area)
            .caption("3D Plot Test".to_string(), ("sans", 20))
            .build_cartesian_3d(x_axis.clone(), -3.0..3.0, z_axis.clone())?;

        chart.with_projection(|mut pb| {
            pb.yaw = 0.5;
            pb.scale = 0.9;
            pb.into_matrix()
        });

        chart
            .configure_axes()
            .light_grid_style(BLACK.mix(0.15))
            .max_light_lines(3)
            .draw()?;

        chart
            .draw_series(
                SurfaceSeries::xoz(
                    (-30..30).map(|f| f as f64 / 10.0),
                    (-30..30).map(|f| f as f64 / 10.0),
                    |x, z| (x * x + z * z).cos(),
                )
                .style(BLUE.mix(0.2).filled()),
            )?
            .label("Surface")
            .legend(|(x, y)| {
                Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], BLUE.mix(0.5).filled())
            });

        chart
            .draw_series(LineSeries::new(
                (-100..100)
                    .map(|y| y as f64 / 40.0)
                    .map(|y| ((y * 10.0).sin(), y, (y * 10.0).cos())),
                &BLACK,
            ))?
            .label("Line")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLACK));

        chart.configure_series_labels().border_style(BLACK).draw()?;

        // To avoid the IO failure being ignored silently, we manually call the present function
        drawing_area.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
        println!("Result has been saved to {}", OUT_FILE_NAME);

        let simulator = DirectSimulator::new(0);
        assert_eq!(simulator.sphere_volume_samples.len(), 0);
    }
}
