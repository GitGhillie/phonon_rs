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

use crate::callbacks::{
    create_callback, get_data_callback, get_float_callback, process_callback, release_callback,
    set_data_callback, set_float_callback, shouldiprocess_callback, sys_deregister_callback,
    sys_register_callback,
};
use glam::Vec3;
use libfmod::ffi::{
    FMOD_DSP_DESCRIPTION, FMOD_DSP_PAN_3D_ROLLOFF_TYPE, FMOD_DSP_PARAMETER_3DATTRIBUTES,
    FMOD_DSP_PARAMETER_ATTENUATION_RANGE, FMOD_DSP_PARAMETER_DATA_TYPE_3DATTRIBUTES,
    FMOD_DSP_PARAMETER_DESC, FMOD_DSP_PARAMETER_DESC_BOOL, FMOD_DSP_PARAMETER_DESC_DATA,
    FMOD_DSP_PARAMETER_DESC_UNION, FMOD_DSP_PARAMETER_OVERALLGAIN, FMOD_DSP_PARAMETER_TYPE_BOOL,
    FMOD_DSP_PARAMETER_TYPE_DATA, FMOD_PLUGIN_SDK_VERSION,
};
use phonon::audio_buffer::AudioBuffer;
use phonon::direct_effect::{DirectEffect, TransmissionType};
use phonon::panning_effect::{PanningEffect, PanningEffectParameters};
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr::null_mut;

enum ParameterApplyType {
    Disable,
    SimulationDefined,
    UserDefine,
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
        let mut num_samples = length * channels;

        // update parameters
        let position = self.source.relative.position;
        let direction = Vec3::new(position.x, position.y, position.z);
        let panning_params = PanningEffectParameters { direction };

        // do the actual processing
        self.in_buffer_stereo.read_interleaved(in_buffer);
        self.in_buffer_stereo.downmix(&mut self.in_buffer_mono);
        self.in_buffer_mono.scale(1.0);

        self.panning_effect
            .apply(panning_params, &self.in_buffer_mono, &mut self.out_buffer);

        self.out_buffer.write_interleaved(out_buffer);
    }
}

pub fn create_dsp_description() -> FMOD_DSP_DESCRIPTION {
    //todo make function to fill in the parameter fields.

    static DESCRIPTION_SOURCE: &str = "Position of the source.\0"; // todo check if this is the correct way
    let param_source = Box::new(FMOD_DSP_PARAMETER_DESC {
        type_: FMOD_DSP_PARAMETER_TYPE_DATA,
        name: str_to_c_char_array("Enable"),
        label: str_to_c_char_array("Yes"),
        description: DESCRIPTION_SOURCE.as_ptr() as *const c_char,
        union: FMOD_DSP_PARAMETER_DESC_UNION {
            datadesc: FMOD_DSP_PARAMETER_DESC_DATA {
                datatype: FMOD_DSP_PARAMETER_DATA_TYPE_3DATTRIBUTES,
            },
        },
    });

    let mut parameters: [*mut FMOD_DSP_PARAMETER_DESC; 1] = [Box::into_raw(param_source)];

    FMOD_DSP_DESCRIPTION {
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
        numparameters: 1,
        paramdesc: parameters.as_mut_ptr(),
        setparameterfloat: None,
        setparameterint: None,
        setparameterbool: None, //todo
        setparameterdata: Some(set_data_callback),
        getparameterfloat: None,
        getparameterint: None,
        getparameterbool: None, // todo
        getparameterdata: Some(get_data_callback),
        shouldiprocess: None,
        userdata: null_mut(),
        sys_register: None,
        sys_deregister: None,
        sys_mix: None,
    }
}

/// FMOD will call this function load the plugin defined by FMOD_DSP_DESCRIPTION.
/// See https://fmod.com/docs/2.02/api/white-papers-dsp-plugin-api.html#building-a-plug-in
#[no_mangle]
extern "C" fn FMODGetDSPDescription() -> *mut FMOD_DSP_DESCRIPTION {
    let desc = Box::new(create_dsp_description());
    Box::into_raw(desc)
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
