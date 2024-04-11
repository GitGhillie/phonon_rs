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

use crate::hrtf_map::HrtfMap;

use ndarray::{Array1, Array2, Array3};
use num::complex::Complex;
use rustfft::Fft;

use std::sync::Arc;

// port notes:
// + FFT becomes Arc<dyn Fft<f32>> because of how RustFFT works

// todo: Document me
pub struct HrtfDatabase {
    // todo: Document me
    pub dcc_correction: bool,
    // todo: Document me
    pub nyquist_correction: bool,
    // todo: Document me
    sampling_rate: i32,
    /// Reference loundess of front HRIR.
    reference_loudness: f32,
    /// Map containing loaded hrtf data.
    data: HrtfMap,
    /// FFT for interpolation and min-phase conversion.
    ///
    /// `samples -> spectrum_samples`.
    interpolation: Arc<dyn Fft<f32>>,
    /// FFT for audio processing.
    ///
    /// ```
    /// padded_samples -> padded_spectrum_samples
    /// where padded_samples = windowd_frame_samples + samples - 1
    /// ```
    audio_processing: Arc<dyn Fft<f32>>,
    /// Head-relative transfer functions (HRTFs).
    ///
    /// Shape: `ears * measurements * padded_spectrum_samples`.
    hrtf: Array3<Complex<f32>>,
    /// Index of peaks in heach HIRI
    ///
    /// Shape: `ears * measurements`.
    peak_delay: Array2<i32>,
    /// HRTF magnitude
    ///
    /// Shape: `ears * measurements * spectrum_samples`.
    hrtf_magnitude: Array3<f32>,
    /// HRTF phase (unwrapped).
    ///
    /// Shape: `ears * measurements * spectrum_samples`.
    hrtf_phase: Array3<f32>,
    /// Temp. storage for interpolated HRTF magnitude.
    ///
    /// Shape: `spectrum_samples`.
    interpolated_hrtf_magnitude: Array1<f32>,
    /// Temp. storage for interpolated HRTF phase.
    ///
    /// Shape: `spectrum_samples`.
    interpolated_hrtf_phase: Array1<f32>,
    /// Interpolated HRTF.
    ///
    /// Shape: `ears * spectrum_samples`.
    interpolated_hrtf: Array2<Complex<f32>>,
    /// Interpolated HRIRs.
    ///
    /// Shape: `ears * padded_samples`
    // todo: Check
    interpolated_hrir: Array1<f32>,
    /// Ambisonics HRTFs.
    ///
    /// Shape: `ears * coefficients * padded_spectrum_samples`.
    ambisonic_hrtf: Array3<Complex<f32>>,
}
