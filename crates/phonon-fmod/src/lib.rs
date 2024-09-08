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

//! FMOD Plugin for the phonon crate.

pub(crate) mod callbacks;
mod fmod_state;
mod parameter_init;
pub mod parameter_spec;

use crate::callbacks::{
    create_callback, get_data_callback, get_int_callback, process_callback, release_callback,
    set_data_callback, set_int_callback, sys_deregister_callback, sys_register_callback,
};
use crate::parameter_init::init_parameters;
use glam::Vec3;
use libfmod::ffi::{
    FMOD_DSP_DESCRIPTION, FMOD_DSP_PAN_3D_ROLLOFF_TYPE, FMOD_DSP_PARAMETER_3DATTRIBUTES,
    FMOD_DSP_PARAMETER_ATTENUATION_RANGE, FMOD_DSP_PARAMETER_OVERALLGAIN, FMOD_PLUGIN_SDK_VERSION,
};
use libfmod::DspDescription;
use phonon::audio_buffer::AudioBuffer;
use phonon::direct_effect::{
    DirectApplyFlags, DirectEffect, DirectEffectParameters, TransmissionType,
};
use phonon::direct_simulator::DirectSoundPath;
use phonon::panning_effect::{PanningEffect, PanningEffectParameters};
use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::ptr::null_mut;

#[derive(Copy, Clone)]
enum ParameterApplyType {
    Disable,
    SimulationDefined,
    UserDefined,
}

impl From<c_int> for ParameterApplyType {
    fn from(value: c_int) -> Self {
        match value {
            0 => ParameterApplyType::Disable,
            1 => ParameterApplyType::SimulationDefined,
            2 => ParameterApplyType::UserDefined,
            _ => ParameterApplyType::Disable,
        }
    }
}

impl Into<c_int> for ParameterApplyType {
    fn into(self) -> c_int {
        match self {
            ParameterApplyType::Disable => 0,
            ParameterApplyType::SimulationDefined => 1,
            ParameterApplyType::UserDefined => 2,
        }
    }
}

pub(crate) struct EffectState {
    source: FMOD_DSP_PARAMETER_3DATTRIBUTES,
    overall_gain: FMOD_DSP_PARAMETER_OVERALLGAIN,

    apply_distance_attenuation: ParameterApplyType,
    apply_air_absorption: ParameterApplyType,
    apply_directivity: ParameterApplyType,
    apply_occlusion: ParameterApplyType,
    apply_transmission: ParameterApplyType,

    distance_attenuation: f32,
    distance_attenuation_rolloff_type: FMOD_DSP_PAN_3D_ROLLOFF_TYPE,
    distance_attenuation_min_distance: f32,
    distance_attenuation_max_distance: f32,

    // todo: I added this one. Consider another one for user settings and then remove all the individual params below.
    direct_sound_path: DirectSoundPath,

    air_absorption: [f32; 3],
    directivity: f32,
    dipole_weight: f32, // See Directivity docs
    dipole_power: f32,  // See Directivity docs
    occlusion: f32,
    transmission_type: TransmissionType,
    transmission: [f32; 3],

    attenuation_range: FMOD_DSP_PARAMETER_ATTENUATION_RANGE,
    attenuation_range_set: bool, // todo: Original is atomic

    in_buffer_stereo: AudioBuffer<2>,
    in_buffer_mono: AudioBuffer<1>,
    out_buffer: AudioBuffer<2>,
    direct_buffer: AudioBuffer<1>,
    mono_buffer: AudioBuffer<1>,

    panning_effect: PanningEffect,
    direct_effect: DirectEffect,
}

impl EffectState {
    fn process(
        &mut self,
        in_buffer: &[f32],
        out_buffer: &mut [f32],
        length: usize,
        channels: usize,
    ) {
        let _num_samples = length * channels;

        // update parameters
        let position = self.source.relative.position;
        let direction = Vec3::new(position.x, position.y, position.z);
        let panning_params = PanningEffectParameters { direction };

        let mut flags = DirectApplyFlags::AirAbsorption
            | DirectApplyFlags::Occlusion
            | DirectApplyFlags::Transmission;

        match self.apply_distance_attenuation {
            ParameterApplyType::Disable => flags.set(DirectApplyFlags::DistanceAttenuation, false),
            ParameterApplyType::SimulationDefined => {
                flags.set(DirectApplyFlags::DistanceAttenuation, true)
            }
            ParameterApplyType::UserDefined => {
                // todo
                flags.set(DirectApplyFlags::DistanceAttenuation, true)
            }
        }

        let direct_params = DirectEffectParameters {
            direct_sound_path: self.direct_sound_path,
            flags,
            transmission_type: TransmissionType::FrequencyDependent,
        };

        // do the actual processing
        self.in_buffer_stereo.read_interleaved(in_buffer);
        self.in_buffer_stereo.downmix(&mut self.in_buffer_mono);

        self.direct_effect
            .apply(direct_params, &self.in_buffer_mono, &mut self.direct_buffer);

        self.panning_effect
            .apply(panning_params, &self.direct_buffer, &mut self.out_buffer);

        self.out_buffer.write_interleaved(out_buffer);
    }
}

pub fn create_dsp_description() -> DspDescription {
    DspDescription {
        pluginsdkversion: FMOD_PLUGIN_SDK_VERSION,
        name: str_to_c_char_array("Phonon Spatializer"),
        version: 1,
        numinputbuffers: 1,
        numoutputbuffers: 1,
        create: Some(create_callback),
        release: Some(release_callback),
        reset: None,
        read: None,
        process: Some(process_callback),
        setposition: None,
        paramdesc: init_parameters(),
        setparameterfloat: None,
        setparameterint: Some(set_int_callback),
        setparameterbool: None, //todo
        setparameterdata: Some(set_data_callback),
        getparameterfloat: None,
        getparameterint: Some(get_int_callback),
        getparameterbool: None, // todo
        getparameterdata: Some(get_data_callback),
        shouldiprocess: None,
        userdata: null_mut(),
        sys_register: Some(sys_register_callback),
        sys_deregister: Some(sys_deregister_callback),
        sys_mix: None,
    }
}

/// FMOD will call this function load the plugin defined by FMOD_DSP_DESCRIPTION.
/// See https://fmod.com/docs/2.02/api/white-papers-dsp-plugin-api.html#building-a-plug-in
#[no_mangle]
extern "C" fn FMODGetDSPDescription() -> *mut FMOD_DSP_DESCRIPTION {
    let description: FMOD_DSP_DESCRIPTION = create_dsp_description().into();
    let boxed = Box::new(description);
    Box::into_raw(boxed)
}

fn str_to_c_char_array<const LEN: usize>(input: &str) -> [c_char; LEN] {
    let mut array: [c_char; LEN] = [0; LEN];

    // Convert the input &str to a CString, adding a null terminator
    let c_string = CString::new(input).expect("CString::new failed");

    // Get the byte slice of the CString
    let bytes = c_string.as_bytes();

    // Ensure the byte slice fits within the array
    if bytes.len() > LEN {
        panic!("String is too long to fit in [c_char; LEN]");
    }

    // Copy the bytes into the array
    for (i, &byte) in bytes.iter().enumerate() {
        array[i] = byte as c_char;
    }

    array
}
