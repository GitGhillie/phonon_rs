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
    create_callback, get_data_callback, process_callback, release_callback, set_data_callback,
    sys_deregister_callback, sys_register_callback,
};
use glam::Vec3;
use libfmod::ffi::{
    FMOD_DSP_DESCRIPTION, FMOD_DSP_PAN_3D_ROLLOFF_TYPE, FMOD_DSP_PARAMETER_3DATTRIBUTES,
    FMOD_DSP_PARAMETER_ATTENUATION_RANGE, FMOD_DSP_PARAMETER_DATA_TYPE,
    FMOD_DSP_PARAMETER_DATA_TYPE_3DATTRIBUTES, FMOD_DSP_PARAMETER_DATA_TYPE_OVERALLGAIN,
    FMOD_DSP_PARAMETER_DESC, FMOD_DSP_PARAMETER_DESC_DATA, FMOD_DSP_PARAMETER_DESC_INT,
    FMOD_DSP_PARAMETER_DESC_UNION, FMOD_DSP_PARAMETER_OVERALLGAIN, FMOD_DSP_PARAMETER_TYPE_DATA,
    FMOD_DSP_PARAMETER_TYPE_INT, FMOD_PLUGIN_SDK_VERSION,
};
use phonon::audio_buffer::AudioBuffer;
use phonon::direct_effect::{DirectEffect, TransmissionType};
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
//
// impl From<c_int> for ParameterApplyType {
//     fn from(value: c_int) -> Self {
//         match value {
//             0 => ParameterApplyType::Disable,
//             1 => ParameterApplyType::SimulationDefined,
//             2 => ParameterApplyType::UserDefined,
//             _ => ParameterApplyType::Disable,
//         }
//     }
// }
//
// impl Into<c_int> for ParameterApplyType {
//     fn into(self) -> c_int {
//         match self {
//             ParameterApplyType::Disable => 0,
//             ParameterApplyType::SimulationDefined => 1,
//             ParameterApplyType::UserDefined => 2,
//         }
//     }
// }

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
        let num_samples = length * channels;

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

fn create_param_data(
    name: &str,
    description: &'static str,
    datatype: FMOD_DSP_PARAMETER_DATA_TYPE,
) -> Box<FMOD_DSP_PARAMETER_DESC> {
    Box::new(FMOD_DSP_PARAMETER_DESC {
        type_: FMOD_DSP_PARAMETER_TYPE_DATA,
        name: str_to_c_char_array(name),
        label: str_to_c_char_array(""),
        description: description.as_ptr() as *const c_char,
        union: FMOD_DSP_PARAMETER_DESC_UNION {
            datadesc: FMOD_DSP_PARAMETER_DESC_DATA { datatype },
        },
    })
}

fn create_param_int(
    name: &str,
    description: &'static str,
    value_names: &'static [&'static str; 3],
) -> Box<FMOD_DSP_PARAMETER_DESC> {
    Box::new(FMOD_DSP_PARAMETER_DESC {
        type_: FMOD_DSP_PARAMETER_TYPE_INT,
        name: str_to_c_char_array(name),
        label: str_to_c_char_array("aa"),
        description: description.as_ptr() as *const c_char,
        union: FMOD_DSP_PARAMETER_DESC_UNION {
            intdesc: FMOD_DSP_PARAMETER_DESC_INT {
                min: 0,
                max: 2,
                defaultval: 0,
                goestoinf: 0,
                valuenames: null_mut(), //value_names.as_ptr() as *const *const c_char,
            },
        },
    })
}

pub fn create_dsp_description() -> FMOD_DSP_DESCRIPTION {
    let param_source = create_param_data(
        "SourcePos",
        "Position of the source.\0",
        FMOD_DSP_PARAMETER_DATA_TYPE_3DATTRIBUTES,
    );
    let param_overall_gain = create_param_data(
        "OverallGain",
        "Overall gain.\0",
        FMOD_DSP_PARAMETER_DATA_TYPE_OVERALLGAIN,
    );
    // let param_apply_da = create_param_int(
    //     "ApplyDA",
    //     "Apply distance attenuation.\0",
    //     &["Off", "Physics-Based", "Curve-Driven"],
    // );

    const NUM_PARAMETERS: usize = 2;
    let parameters: Box<[*mut FMOD_DSP_PARAMETER_DESC; NUM_PARAMETERS]> = Box::new([
        Box::into_raw(param_source),
        Box::into_raw(param_overall_gain),
        //Box::into_raw(param_apply_da),
    ]);

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
        numparameters: NUM_PARAMETERS as c_int,
        paramdesc: Box::into_raw(parameters) as *mut *mut FMOD_DSP_PARAMETER_DESC,
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
        sys_register: Some(sys_register_callback),
        sys_deregister: Some(sys_deregister_callback),
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
