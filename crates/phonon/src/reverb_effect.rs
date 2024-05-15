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

use crate::audio_buffer::{AudioBuffer, AudioEffectState, AudioSettings};
use crate::bands::NUM_BANDS;
use crate::delay::Delay;
use crate::reverb_estimator::Reverb;

use crate::iir::{IIRFilterer, IIR};
use derive_deref::{Deref, DerefMut};
use rand::Rng;
use ultraviolet::f32x4;

const NUM_DELAYS: usize = 16;
const NUM_ALLPASS_DELAYS: i32 = 4;
const TONE_CORRECTION_WEIGHT: f32 = 0.5;

const ALLPASS_DELAYS: [usize; 4] = [225, 341, 441, 556];

#[derive(Deref, DerefMut)]
pub struct ReverbEffectParams(Reverb);

pub struct ReverbEffect {
    sampling_rate: i32,
    frame_size: usize,
    delay_values: [i32; NUM_DELAYS],
    delay_lines: [Delay; NUM_DELAYS],
    current: usize,
    is_first_frame: bool,
    allpass_x: [[Delay; 2]; NUM_DELAYS],
    allpass_y: [[Delay; 2]; NUM_DELAYS],
    absorptive: Vec<Vec<Vec<IIRFilterer>>>, //todo perf
    tone_correction: Vec<Vec<IIRFilterer>>,
    x_old: Vec<Vec<f32>>,
    x_new: Vec<Vec<f32>>,
    previous_reverb: Reverb,
    num_tail_frames_remaining: i32,
}

impl ReverbEffect {
    pub fn new(audio_settings: AudioSettings) -> Self {
        let delay_values = Self::calc_delays_for_reverb_time(2.0, audio_settings.sampling_rate);

        let delay_lines: [Delay; NUM_DELAYS] = core::array::from_fn::<_, NUM_DELAYS, _>(|i| {
            Delay::new(ALLPASS_DELAYS[i % 4], audio_settings.frame_size)
        });

        let allpass_x: [[Delay; 2]; NUM_DELAYS] = core::array::from_fn::<_, NUM_DELAYS, _>(|i| {
            [
                Delay::new(ALLPASS_DELAYS[i % 4], 0),
                Delay::new(ALLPASS_DELAYS[(i + 1) % 4], 0),
            ]
        });

        let allpass_y: [[Delay; 2]; NUM_DELAYS] = core::array::from_fn::<_, NUM_DELAYS, _>(|i| {
            [
                Delay::new(ALLPASS_DELAYS[i % 4], 0),
                Delay::new(ALLPASS_DELAYS[(i + 1) % 4], 0),
            ]
        });

        Self {
            sampling_rate: audio_settings.sampling_rate,
            frame_size: audio_settings.frame_size,
            delay_values,
            delay_lines,
            current: 0,
            is_first_frame: false,
            allpass_x,
            allpass_y,
            absorptive: Vec::default(),
            tone_correction: Vec::default(),
            x_old: vec![vec![0.0; audio_settings.frame_size]; NUM_DELAYS],
            x_new: vec![vec![0.0; audio_settings.frame_size]; NUM_DELAYS],
            previous_reverb: Reverb::default(),
            num_tail_frames_remaining: 0,
        }

        //todo: reset ReverbEffect?
    }

    fn tail_float4(&self, out: &mut [f32]) {
        // for i in 0..NUM_DELAYS {
        //     self.delay_lines[i].get(self.frame_size, self.x_old[i]);
        // }
    }

    fn apply_float32x4(&mut self, reverb_times: &[f32], input: &[f32], output: &mut [f32]) {
        // todo profile function

        let clamped_reverb_times =
            core::array::from_fn::<_, NUM_BANDS, _>(|i| reverb_times[i].max(0.1));

        output.fill(0.0);

        const LOW_CUTOFF: [f32; NUM_BANDS] = [20.0, 500.0, 5_000.0];
        const HIGH_CUTOFF: [f32; NUM_BANDS] = [500.0, 5_000.0, 22_000.0];

        for i in 0..NUM_DELAYS {
            let absorptive_gains = core::array::from_fn::<_, NUM_BANDS, _>(|j| {
                Self::calc_absorptive_gain(self, clamped_reverb_times[j], self.delay_values[i])
            });

            let iir = [
                IIR::new_low_shelf(HIGH_CUTOFF[0], absorptive_gains[0], self.sampling_rate),
                IIR::new_peaking(
                    LOW_CUTOFF[1],
                    HIGH_CUTOFF[1],
                    absorptive_gains[1],
                    self.sampling_rate,
                ),
                IIR::new_low_shelf(LOW_CUTOFF[2], absorptive_gains[2], self.sampling_rate),
            ];

            for j in 0..NUM_BANDS {
                self.absorptive[i][j][self.current] = IIRFilterer::new(iir[j]);
            }
        }

        let mut tone_correction_gains = [0.0f32; NUM_BANDS];
        Self::calc_tone_correction_gains(&clamped_reverb_times, &mut tone_correction_gains);

        let iir = [
            IIR::new_low_shelf(HIGH_CUTOFF[0], tone_correction_gains[0], self.sampling_rate),
            IIR::new_peaking(
                LOW_CUTOFF[1],
                HIGH_CUTOFF[1],
                tone_correction_gains[1],
                self.sampling_rate,
            ),
            IIR::new_low_shelf(LOW_CUTOFF[2], tone_correction_gains[2], self.sampling_rate),
        ];

        for i in 0..NUM_BANDS {
            self.tone_correction[i][self.current] = IIRFilterer::new(iir[i]);
        }

        for i in 0..NUM_DELAYS {
            self.delay_lines[i].get(self.frame_size, self.x_old[i].as_mut_slice());
        }

        let mut x_old = [f32x4::ZERO; NUM_DELAYS];
        let mut x_new = [f32x4::ZERO; NUM_DELAYS];
        for i in (0..self.frame_size).step_by(4) {
            for j in 0..NUM_DELAYS {
                x_old[j] = f32x4::from(&self.x_old[j][i..i + 4]);
            }

            Self::multiply_hadamard_matrix(x_old.as_slice(), x_new.as_mut_slice());

            for j in 0..NUM_DELAYS {
                self.x_new[j][i..i + 4].copy_from_slice(x_new[j].as_array_ref());
            }
        }

        for i in 0..NUM_DELAYS {
            for j in 0..NUM_BANDS {
                // todo: perf?
                let copy = self.x_new[i].clone();

                self.absorptive[i][j][self.current].apply(
                    self.frame_size,
                    copy.as_slice(),
                    self.x_new[i].as_mut_slice(),
                );
            }

            // Element-wise addition the `input` of this function to self.x_new[i]
            // todo: Consider using ndarray for this
            input
                .into_iter()
                .zip(self.x_new[i].iter_mut())
                .for_each(|(i, o)| *o += *i);

            self.delay_lines[i].put(self.frame_size, self.x_new[i].as_slice());
        }

        // for...
    }

    fn calc_delays_for_reverb_time(reverb_time: f32, sampling_rate: i32) -> [i32; NUM_DELAYS] {
        let mut result: [i32; NUM_DELAYS] = [0; NUM_DELAYS];

        const PRIMES: [i32; 16] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53];

        let delay_sum = 0.15 * reverb_time * (sampling_rate as f32);
        let delay_min = (delay_sum / (NUM_DELAYS as f32)) as i32;

        // todo: Check if rand performance and behavior is OK.
        let mut rng = rand::thread_rng();

        for i in 0..NUM_DELAYS {
            let random_offset_unsigned: u16 = rng.gen();
            let random_offset = (random_offset_unsigned as i32) % 101;

            result[i] = Self::next_power_of_prime(delay_min + random_offset, PRIMES[i]);
        }

        result
    }

    // todo: reverb_times should be const
    fn calc_absorptive_gains(&self, reverb_times: &[f32], delay: i32, gains: &mut [f32]) {
        for i in 0..NUM_BANDS {
            let nominator = 6.91 * delay as f32;
            let denominator = reverb_times[i] * self.sampling_rate as f32;

            gains[i] = (-nominator / denominator).exp();
        }
    }

    // todo: reverb_times should be const
    fn calc_absorptive_gain(&self, reverb_time: f32, delay: i32) -> f32 {
        let nominator = 6.91 * delay as f32;
        let denominator = reverb_time * self.sampling_rate as f32;

        (-nominator / denominator).exp()
    }

    // todo: reverb_times should be const
    fn calc_tone_correction_gains(reverb_times: &[f32], gains: &mut [f32]) {
        for i in 0..NUM_BANDS {
            gains[i] = (1.0 / reverb_times[i]).sqrt();
        }

        let max_gain = gains[0].max(gains[1]).max(gains[2]);
        for gain in gains.iter_mut() {
            *gain /= max_gain;
        }
    }

    #[rustfmt::skip]
    fn multiply_hadamard_matrix(x: &[f32x4], y: &mut [f32x4]) {
        y[0]  = x[0] + x[1] + x[2] + x[3] + x[4] + x[5] + x[6] + x[7] + x[8] + x[9] + x[10] + x[11] + x[12] + x[13] + x[14] + x[15];
        y[1]  = x[0] - x[1] + x[2] - x[3] + x[4] - x[5] + x[6] - x[7] + x[8] - x[9] + x[10] - x[11] + x[12] - x[13] + x[14] - x[15];
        y[2]  = x[0] + x[1] - x[2] - x[3] + x[4] + x[5] - x[6] - x[7] + x[8] + x[9] - x[10] - x[11] + x[12] + x[13] - x[14] - x[15];
        y[3]  = x[0] - x[1] - x[2] + x[3] + x[4] - x[5] - x[6] + x[7] + x[8] - x[9] - x[10] + x[11] + x[12] - x[13] - x[14] + x[15];
        y[4]  = x[0] + x[1] + x[2] + x[3] - x[4] - x[5] - x[6] - x[7] + x[8] + x[9] + x[10] + x[11] - x[12] - x[13] - x[14] - x[15];
        y[5]  = x[0] - x[1] + x[2] - x[3] - x[4] + x[5] - x[6] + x[7] + x[8] - x[9] + x[10] - x[11] - x[12] + x[13] - x[14] + x[15];
        y[6]  = x[0] + x[1] - x[2] - x[3] - x[4] - x[5] + x[6] + x[7] + x[8] + x[9] - x[10] - x[11] - x[12] - x[13] + x[14] + x[15];
        y[7]  = x[0] - x[1] - x[2] + x[3] - x[4] + x[5] + x[6] - x[7] + x[8] - x[9] - x[10] + x[11] - x[12] + x[13] + x[14] - x[15];
        y[8]  = x[0] + x[1] + x[2] + x[3] + x[4] + x[5] + x[6] + x[7] - x[8] - x[9] - x[10] - x[11] - x[12] - x[13] - x[14] - x[15];
        y[9]  = x[0] - x[1] + x[2] - x[3] + x[4] - x[5] + x[6] - x[7] - x[8] + x[9] - x[10] + x[11] - x[12] + x[13] - x[14] + x[15];
        y[10] = x[0] + x[1] - x[2] - x[3] + x[4] + x[5] - x[6] - x[7] - x[8] - x[9] + x[10] + x[11] - x[12] - x[13] + x[14] + x[15];
        y[11] = x[0] - x[1] - x[2] + x[3] + x[4] - x[5] - x[6] + x[7] - x[8] + x[9] + x[10] - x[11] - x[12] + x[13] + x[14] - x[15];
        y[12] = x[0] + x[1] + x[2] + x[3] - x[4] - x[5] - x[6] - x[7] - x[8] - x[9] - x[10] - x[11] + x[12] + x[13] + x[14] + x[15];
        y[13] = x[0] - x[1] + x[2] - x[3] - x[4] + x[5] - x[6] + x[7] - x[8] + x[9] - x[10] + x[11] + x[12] - x[13] + x[14] - x[15];
        y[14] = x[0] + x[1] - x[2] - x[3] - x[4] - x[5] + x[6] + x[7] - x[8] - x[9] + x[10] + x[11] + x[12] + x[13] - x[14] - x[15];
        y[15] = x[0] - x[1] - x[2] + x[3] - x[4] + x[5] + x[6] - x[7] - x[8] + x[9] + x[10] - x[11] + x[12] - x[13] - x[14] + x[15];

        for i in 0..NUM_DELAYS {
            y[i] = y[i] * 0.25;
        }
    }

    //todo test if correct
    fn next_power_of_prime(x: i32, p: i32) -> i32 {
        let x_float = x as f32;
        let p_float = p as f32;

        let m = (x_float.ln() / p_float.ln()).round() as u32;
        p.pow(m)
    }
}
