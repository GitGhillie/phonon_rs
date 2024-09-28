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

use crate::dsp::audio_buffer::{AudioBuffer, AudioEffectState, AudioSettings};
use crate::dsp::bands::NUM_BANDS;
use bitflags::bitflags;
use std::cmp::PartialEq;

use crate::effects::eq::{EqEffect, EqEffectParameters};
use crate::effects::gain::{GainEffect, GainEffectParameters};
use crate::simulators::direct::DirectSoundPath;

bitflags! {
    //todo check if these are all necessary
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DirectApplyFlags: u8 {
        const DistanceAttenuation = 1 << 0;
        const AirAbsorption = 1 << 1;
        const Directivity = 1 << 2;
        const Occlusion = 1 << 3;
        const Transmission = 1 << 4;
        const Delay = 1 << 5;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransmissionType {
    FrequencyIndependent,
    FrequencyDependent,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectEffectParameters {
    pub direct_sound_path: DirectSoundPath,
    pub flags: DirectApplyFlags,
    pub transmission_type: TransmissionType,
}

// Port note: Compared to the original code this DirectEffect applies to 1 channel only.
/// Audio effect that applies direct sound path parameters to an incoming multichannel audio buffer.
pub struct DirectEffect {
    pub frame_size: usize,
    /// One filter object per channel to apply effect.
    eq_effect: EqEffect,
    /// Attenuation interpolation.
    gain_effect: GainEffect,
}

impl DirectEffect {
    pub fn new(audio_settings: AudioSettings) -> Self {
        Self {
            frame_size: audio_settings.frame_size,
            eq_effect: EqEffect::new(audio_settings),
            gain_effect: GainEffect::new(audio_settings),
        }
    }

    pub(crate) fn reset(&mut self) {
        self.eq_effect.reset();
        self.gain_effect.reset();
    }

    pub fn apply(
        &mut self,
        parameters: DirectEffectParameters,
        input: &AudioBuffer<1>,
        output: &mut AudioBuffer<1>,
    ) -> AudioEffectState {
        let mut gain: f32 = 0.0;
        let mut eq_coefficients: [f32; NUM_BANDS] = [0.0, 0.0, 0.0];
        // todo perf: This does not exist in the original code.
        let mut buf: AudioBuffer<1> = AudioBuffer::new(input[0].len());

        // todo: This function should just take a DirectEffectParameters?
        Self::calculate_gain_and_eq(
            parameters.direct_sound_path,
            parameters.flags,
            parameters.transmission_type,
            &mut gain,
            &mut eq_coefficients,
        );

        let air_absorption = (parameters.flags & DirectApplyFlags::AirAbsorption).bits() != 0;
        let transmission = (parameters.flags & DirectApplyFlags::Transmission).bits() != 0;
        let transmission_freq_dep =
            parameters.transmission_type == TransmissionType::FrequencyDependent;
        let apply_eq = air_absorption || (transmission && transmission_freq_dep);

        let gain_parameters = GainEffectParameters { gain };

        if apply_eq {
            let eq_parameters = EqEffectParameters {
                gains: eq_coefficients,
            };

            self.eq_effect.apply(eq_parameters, input, &mut buf);
            self.gain_effect.apply(gain_parameters, &buf, output);
        } else {
            self.gain_effect.apply(gain_parameters, input, output);
        }

        AudioEffectState::TailComplete
    }

    pub(crate) fn tail(output: &mut AudioBuffer<1>) -> AudioEffectState {
        output.make_silent();
        AudioEffectState::TailComplete
    }

    fn calculate_gain_and_eq(
        direct_path: DirectSoundPath,
        flags: DirectApplyFlags,
        transmission_type: TransmissionType,
        overall_gain: &mut f32,
        eq_coefficients: &mut [f32; NUM_BANDS],
    ) {
        // Apply distance attenuation.
        *overall_gain = match flags & DirectApplyFlags::DistanceAttenuation {
            DirectApplyFlags::DistanceAttenuation => direct_path.distance_attenuation,
            _ => 1.0,
        };

        // Apply air absorption.
        for i in 0..NUM_BANDS {
            if (flags & DirectApplyFlags::AirAbsorption).bits() != 0 {
                eq_coefficients[i] = direct_path.air_absorption[i];
            } else {
                eq_coefficients[i] = 1.0;
            }
        }

        // Apply directivity.
        if (flags & DirectApplyFlags::Directivity).bits() != 0 {
            *overall_gain *= direct_path.directivity;
        }

        if ((flags & DirectApplyFlags::AirAbsorption).bits() != 0)
            || (((flags & DirectApplyFlags::Transmission).bits() != 0)
                && transmission_type == TransmissionType::FrequencyDependent)
        {
            // Maximum value in EQ filter should be normalized to 1 and common factor rolled into attenuation factor,
            // this will allow for smooth changes to frequency changes (possible exception is if maximum remains
            // and low / mid-frequencies change dramatically). Minimum value should be .0625 (24 dB) for any frequency
            // band for a good EQ response.
            EqEffect::normalize_gains(eq_coefficients, overall_gain);
        }

        // Early return if we don't apply occlusion
        if (flags & DirectApplyFlags::Occlusion).bits() == 0 {
            return;
        }

        // Apply occlusion and transmission.
        if (flags & DirectApplyFlags::Transmission).bits() != 0 {
            match transmission_type {
                TransmissionType::FrequencyIndependent => {
                    // Update attenuation factor with the average transmission coefficient and
                    // appropriately applied occlusion factor.
                    let mut average_transmission_factor = 0.0;
                    for transmission in direct_path.transmission {
                        average_transmission_factor += transmission;
                    }

                    average_transmission_factor /= NUM_BANDS as f32;

                    *overall_gain *= direct_path.occlusion
                        + (1.0 - direct_path.occlusion) * average_transmission_factor;
                }
                TransmissionType::FrequencyDependent => {
                    // Update per frequency factors based on occlusion and transmission value.
                    for i in 0..NUM_BANDS {
                        eq_coefficients[i] *= direct_path.occlusion
                            + (1.0 - direct_path.occlusion) * direct_path.transmission[i];
                    }
                }
            };
        } else {
            // Update attenuation factor with the occlusion factor only.
            *overall_gain *= direct_path.occlusion;
        }
    }
}
