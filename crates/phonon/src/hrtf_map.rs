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

use std::io::{Read, Result};

use byteorder::{LittleEndian, ReadBytesExt};
use ndarray::{Array1, Array2, Array3};

use crate::polar_vector::{InterauralVec3, SphericalVec3};

const MIN_FILE_FORMAT_VERSION: i32 = 0;
const MAX_FILE_FORMAT_VERSION: i32 = 1;

// A data structure that stores loaded HRTF data and allows nearest-neighbor and
// interpolated queries.
pub struct HrtfMap {
    // todo: document
    version: i32,
    /// Number of mesurement positions.
    num_hrirs: usize,
    /// Number of unique azimuths.
    num_azimuths: usize,
    /// Number of elivations at any given azimuth.
    num_elivations: usize,
    /// Number of samples in an HRIR.
    num_samples: usize,
    /// Azimuth values.
    ///
    /// Shape: `azimuths`.
    azimuths: Array1<f32>,
    /// Elivation values, for each azimuth.
    ///
    /// Shape: `azimuths * elivations`.
    elivations_for_azimuth: Array2<f32>,
    /// HRIRs.
    ///
    /// Shape: `ears * measurements * samples`
    hrir: Array3<f32>,
    /// Ambisonics HRIRs.
    ///
    /// Shape: `ears * coefficients * samples`
    ambisonics_hrir: Array3<f32>,
}

impl HrtfMap {
    fn load<R: Read>(sampling_rate: i32, reader: &mut R) -> Result<HrtfMap> {
        // Read and verify header
        let forcc_id = reader.read_i32::<LittleEndian>()?;
        let version = reader.read_i32::<LittleEndian>()?;
        if version < MIN_FILE_FORMAT_VERSION || MAX_FILE_FORMAT_VERSION < version {
            panic!("Unsupported HRTF data format version {}", version);
        }

        // Load HRIRs
        let num_hrirs = reader.read_i32::<LittleEndian>()?;

        // Load angles and convert from canonical spherical coordinates to
        // interaural spherical coordinates.
        let mut interaural_elevations = Vec::new();
        let mut interaural_azimuths = Vec::new();
        for i in 0..num_hrirs {
            let elivation = reader.read_f32::<LittleEndian>()?.to_radians();
            let azimuth = reader.read_f32::<LittleEndian>()?.to_radians();

            let canonical_coords = SphericalVec3::new(1.0, elivation, azimuth);
            let interaural_coords = InterauralVec3::from(canonical_coords);

            let elivation = interaural_coords.elivation.to_degrees() as i32;
            let azimuth = interaural_coords.elivation.to_degrees() as i32;

            interaural_elevations.push(elivation);
            interaural_azimuths.push(azimuth);
        }

        // The data is assumed to be measured at N distinct azimuth "rings",
        // with each ring containing M distinct measurements at different
        // elevations, giving a total of NM measurements. We want to extract the
        // N azimuth values. The interauralAzimuths array contains the azimuths
        // for each measurement, so each azimuth will be repeated M times in
        // that array. So we sort it, then permute it so that all the unique
        // values are at the start of the array, then copy the N unique azimuths
        // into `azimuths`.

        todo!()
    }
}
