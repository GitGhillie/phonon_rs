//
// Copyright 2017-2023 Valve Corporation.
// Copyright 2026 phonon_rs contributors.
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

use std::sync::Arc;

use rustfft::{Fft, FftPlanner, num_complex::Complex};

use crate::dsp::{audio_buffer::AudioSettings, window_function::tukey};

pub struct OverlapAddConvolutionEffectSettings {
    pub num_channels: u8,
    pub ir_size: usize,
}

// Port note: multipleInputs did not seem to be used, so it is not added here
pub struct OverlapAddConvolutionEffectParams {
    pub fft_impulse_response: Vec<Complex<f32>>,
    //pub multiple_inputs: bool,
}

pub struct OverlapAddConvolutionEffect {
    num_channels: u8,
    impulse_response_size: usize,
    frame_size: usize,
    window: Vec<f32>,
    fft_forward: Arc<dyn Fft<f32>>,
    fft_inverse: Arc<dyn Fft<f32>>,
    windowed_dry: Vec<f32>,
    fft_windowed_dry: Vec<Complex<f32>>,
    dry: Vec<f32>,
    wet: Vec<f32>,
    fft_wet: Vec<Complex<f32>>,
    overlap: Vec<f32>,
    num_tail_samples_remaining: usize,
}

impl OverlapAddConvolutionEffect {
    fn new(
        audio_settings: AudioSettings,
        effect_settings: OverlapAddConvolutionEffectSettings,
    ) -> Self {
        let frame_size = audio_settings.frame_size;
        let window_size = frame_size + (frame_size / 4);
        let fft_size = window_size + effect_settings.ir_size - 1;

        let num_real_samples = fft_size; // Not sure about this one
        let num_complex_samples = fft_size; // Not sure about this one

        let overlap_size = num_real_samples - frame_size;

        // todo: It looks like this one should be reused between different effects
        let mut fft_planner = FftPlanner::new();
        let fft_forward = fft_planner.plan_fft_forward(fft_size);
        let fft_inverse = fft_planner.plan_fft_inverse(fft_size);

        let window_tukey = tukey(frame_size, frame_size / 4);

        Self {
            num_channels: effect_settings.num_channels,
            impulse_response_size: effect_settings.ir_size,
            frame_size,
            window: window_tukey,
            fft_forward,
            fft_inverse,
            windowed_dry: vec![0.0; num_real_samples],
            fft_windowed_dry: vec![Complex::default(); num_complex_samples],
            dry: vec![0.0; window_size],
            wet: vec![0.0; num_real_samples],
            fft_wet: vec![Complex::default(); num_complex_samples],
            overlap: vec![0.0; overlap_size],
            num_tail_samples_remaining: 0,
        }
    }

    fn apply(
        &mut self,
        parameters: OverlapAddConvolutionEffectParams,
        input: &[&[f32]],
        output: &mut [&mut [f32]],
    ) {
        // Steam Audio assertions:
        // num samples in == num sample out
        // num channels in == 1 or self.num_channels
        // num channels out == self.num_channels
        // Assuming one channel in for now

        // Make room for new input
        let overlap_size = self.frame_size / 4;
        let (dry, dry_previous) = self.dry.split_at_mut(self.frame_size);
        dry[0..overlap_size].copy_from_slice(dry_previous);

        // Add input to the dry signal
        self.dry[overlap_size..].copy_from_slice(input[0]);

        // Apply Tukey window to dry
        for i in 0..self.window.len() {
            self.windowed_dry[i] = self.dry[i] * self.window[i];
        }

        // Apply FFT to windowed signal
        for i in 0..self.windowed_dry.len() {
            self.fft_windowed_dry[i].re = self.windowed_dry[i];
        }
        self.fft_forward.process(&mut self.fft_windowed_dry);

        // Convolve
        println!("{}", self.fft_wet.len());
        println!("{}", self.fft_windowed_dry.len());
        println!("{}", parameters.fft_impulse_response.len());
        for i in 0..self.fft_windowed_dry.len() {
            self.fft_wet[i] = self.fft_windowed_dry[i] * parameters.fft_impulse_response[i];
        }

        // Back to time domain
        self.fft_inverse.process(&mut self.fft_wet); // scale by num samples?
        for i in 0..self.wet.len() {
            self.wet[i] = self.fft_wet[i].re;
        }

        // Add previous overlap
        for i in 0..self.overlap.len() {
            self.wet[i] += self.overlap[i];
        }

        // Copy tail to overlap
        self.overlap.copy_from_slice(&self.wet[overlap_size..]);

        // Copy wet to output
        output[0].copy_from_slice(&self.wet);

        // todo: return whether tail samples are remaining
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use plotters::prelude::*;

    #[test]
    fn overlap_add() {
        let frame_size = 1024;
        let sampling_rate = 48_000;
        let audio_settings = AudioSettings::new(sampling_rate, frame_size);

        let fft_size = 2303;
        let mut fft_planner = FftPlanner::new();
        let fft_forward = fft_planner.plan_fft_forward(fft_size);

        // todo magic num
        let mut fft_impulse_response: Vec<Complex<f32>> = vec![Complex::default(); 2303];
        fft_impulse_response[2303 / 2].re = 1.0;
        fft_forward.process(&mut fft_impulse_response);

        let effect_settings = OverlapAddConvolutionEffectSettings {
            num_channels: 1,
            ir_size: 1024, //todo
        };

        let params = OverlapAddConvolutionEffectParams {
            fft_impulse_response,
        };

        let mut effect = OverlapAddConvolutionEffect::new(audio_settings, effect_settings);

        let input: Vec<f32> = (0..frame_size).map(|i| ((i as f32) * 0.1).sin()).collect();
        let mut output: Vec<f32> = vec![0.0; frame_size];

        effect.apply(params, &[&input], &mut [&mut output]);
    }

    #[ignore = "visual check only."]
    #[test]
    fn test_fft_visual() -> Result<(), Box<dyn std::error::Error>> {
        let root = BitMapBackend::new("figures/0.png", (640, 480)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .caption("fft testing", ("sans", 30).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(-1f32..200f32, -2f32..2f32)?;

        chart.configure_mesh().draw()?;

        let fft_size = 100;
        let mut fft_planner = FftPlanner::new();
        let fft_forward = fft_planner.plan_fft_forward(fft_size);
        let fft_inverse = fft_planner.plan_fft_inverse(fft_size);

        let mut input: Vec<Complex<f32>> = Vec::default();

        for i in 0..200 {
            let y = ((i as f32) * 0.1).sin();
            input.push(Complex { re: y, im: 0.0 });
        }

        chart
            .draw_series(LineSeries::new(
                input.iter().enumerate().map(|(a, b)| (a as f32, b.re)),
                &RED,
            ))?
            .label("input")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        // todo process with scratch
        // buffer length must be multiple of fft size
        //
        fft_forward.process(&mut input);

        chart
            .draw_series(LineSeries::new(
                input.iter().enumerate().map(|(a, b)| (a as f32, b.re)),
                &BLUE,
            ))?
            .label("FFT")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

        fft_inverse.process(&mut input);

        chart
            .draw_series(LineSeries::new(
                input
                    .iter()
                    .enumerate()
                    .map(|(a, b)| (a as f32, b.re / 200.0)),
                &GREEN,
            ))?
            .label("output")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        root.present()?;

        Ok(())
    }
}
