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

use crate::delay::Delay;
use biquad::*;

/// Represents a biquad IIR filter, that can be used to carry out various filtering operations on RealSignals. Such a
/// filter is essentially a recurrence relation: sample N of the filtered output signal depends on samples N, N-1, and
/// N-2 of the input, as well as samples N-1 and N-2 of the _output_.
#[derive(Copy, Clone, Debug)]
pub struct IIR(DirectForm1<f32>);

impl IIR {
    /// Creates a low-shelf filter (controls the amplitude of all frequencies below the cutoff).
    pub fn new_low_shelf(high_cutoff: f32, gain: f32, sample_rate: i32) -> Self {
        // Port note: The IIR crate used assumes gain is in dB.
        // This does create some extra work that will hopefully be optimized away...
        let gain_adjusted = (gain.sqrt().log(10.0)) * 40.0;

        let coefficients = Coefficients::<f32>::from_params(
            Type::LowShelf(gain_adjusted),
            sample_rate.hz(),
            high_cutoff.hz(),
            Q_BUTTERWORTH_F32,
        );

        IIR(DirectForm1::<f32>::new(coefficients.unwrap()))
    }

    /// Creates a high-shelf filter (controls the amplitude of all frequencies above the cutoff).
    pub fn new_high_shelf(low_cutoff: f32, gain: f32, sample_rate: i32) -> Self {
        // Port note: The IIR crate used assumes gain is in dB.
        // This does create some extra work that will hopefully be optimized away...
        let gain_adjusted = (gain.sqrt().log(10.0)) * 40.0;

        let coefficients = Coefficients::<f32>::from_params(
            Type::HighShelf(gain_adjusted),
            sample_rate.hz(),
            low_cutoff.hz(),
            Q_BUTTERWORTH_F32,
        );

        IIR(DirectForm1::<f32>::new(coefficients.unwrap()))
    }

    /// Creates a peaking filter (controls the amplitude of all frequencies between the cutoffs).
    pub fn new_peaking(low_cutoff: f32, high_cutoff: f32, gain: f32, sample_rate: i32) -> Self {
        // Port note: The IIR crate used assumes gain is in dB.
        // This does create some extra work that will hopefully be optimized away...
        let gain_adjusted = (gain.sqrt().log(10.0)) * 40.0;
        let cutoff_frequency = (high_cutoff * low_cutoff).sqrt();
        let q_value = cutoff_frequency / (high_cutoff - low_cutoff);

        let coefficients = Coefficients::<f32>::from_params(
            Type::PeakingEQ(gain_adjusted),
            sample_rate.hz(),
            cutoff_frequency.hz(),
            q_value,
        );

        IIR(DirectForm1::<f32>::new(coefficients.unwrap()))
    }

    pub fn new_empty() -> Self {
        IIR(DirectForm1::<f32>::new(Coefficients {
            a1: 0.0,
            a2: 0.0,
            b0: 0.0,
            b1: 0.0,
            b2: 0.0,
        }))
    }
}

/// State required for filtering a signal with an IIR filter over multiple frames. Ensures continuity between frames
/// when the filter doesn't change between frames. If the filter _does_ change, the caller must implement
/// crossfading or some other approach to ensure smoothness.
#[derive(Copy, Clone)]
pub struct IIRFilterer {
    /// The IIR filter to apply.
    filter: IIR,
    /// Input value from 1 sample ago.
    xm1: f32,
    /// Input value from 2 samples ago.
    xm2: f32,
    /// Output value from 1 sample ago.
    ym1: f32,
    /// Output value from 2 samples ago.
    ym2: f32,
}

impl IIRFilterer {
    // todo: With Phonon you can change the filterer at runtime. Let's see if we can get away with not doing that
    pub fn new(filter: IIR) -> Self {
        Self {
            filter,
            xm1: 0.0,
            xm2: 0.0,
            ym1: 0.0,
            ym2: 0.0,
        }
    }

    pub fn set_filter(&mut self, filter: IIR) {
        self.filter = filter;
    }

    pub fn copy_state_from(&mut self, source: IIRFilterer) {
        self.xm1 = source.xm1;
        self.xm2 = source.xm2;
        self.ym1 = source.ym1;
        self.ym2 = source.ym2;
    }

    /// Applies the filter to an entire buffer of input, using SIMD operations.
    pub fn apply(&self, size: usize, input: &[f32], output: &mut [f32]) {
        //self.filter
    }
}
