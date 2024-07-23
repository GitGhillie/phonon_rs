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

use libfmod::ffi::{
    FMOD_BOOL, FMOD_CHANNELMASK, FMOD_DSP_BUFFER_ARRAY, FMOD_DSP_DESCRIPTION,
    FMOD_DSP_PAN_3D_ROLLOFF_TYPE, FMOD_DSP_PARAMETER_3DATTRIBUTES, FMOD_DSP_PARAMETER_DESC,
    FMOD_DSP_PARAMETER_DESC_FLOAT, FMOD_DSP_PARAMETER_DESC_UNION, FMOD_DSP_PARAMETER_FLOAT_MAPPING,
    FMOD_DSP_PARAMETER_OVERALLGAIN, FMOD_DSP_PARAMETER_TYPE_FLOAT, FMOD_DSP_PROCESS_OPERATION,
    FMOD_DSP_PROCESS_PERFORM, FMOD_DSP_PROCESS_QUERY, FMOD_DSP_STATE, FMOD_ERR_DSP_DONTPROCESS,
    FMOD_ERR_INVALID_PARAM, FMOD_ERR_MEMORY, FMOD_OK, FMOD_PLUGIN_SDK_VERSION, FMOD_RESULT,
    FMOD_SPEAKERMODE,
};
use phonon::direct_effect::TransmissionType;
use std::ffi::CString;
use std::mem::MaybeUninit;
use std::os::raw::{c_char, c_float, c_int};
use std::ptr::addr_of_mut;
use std::ptr::null_mut;
use std::slice;

enum ParameterApplyType {
    Disable,
    SimulationDefined,
    UserDefine,
}

struct EffectState {
    source: FMOD_DSP_PARAMETER_3DATTRIBUTES,
    overall_gain: FMOD_DSP_PARAMETER_OVERALLGAIN,

    apply_distance_attenuation: ParameterApplyType,
    apply_air_absorption: ParameterApplyType,
    apply_directivity: ParameterApplyType,
    apply_occlusion: ParameterApplyType,
    apply_transmission: ParameterApplyType,

    // todo some missing fields in between here

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

    // todo remaining fields
}

impl EffectState {}

static mut FMOD_GAIN_STATE: EffectState = EffectState {};

unsafe extern "C" fn create_callback(dsp_state: *mut FMOD_DSP_STATE) -> FMOD_RESULT {
    let struct_ptr: *mut EffectState = addr_of_mut!(FMOD_GAIN_STATE);
    (*dsp_state).plugindata = struct_ptr as *mut std::os::raw::c_void;

    if (*dsp_state).plugindata.is_null() {
        return FMOD_ERR_MEMORY;
    }

    FMOD_OK
}

unsafe extern "C" fn release_callback(dsp_state: *mut FMOD_DSP_STATE) -> FMOD_RESULT {
    (*dsp_state).plugindata = null_mut();
    FMOD_OK
}

unsafe extern "C" fn reset_callback(dsp_state: *mut FMOD_DSP_STATE) -> FMOD_RESULT {
    let state: *mut EffectState = (*dsp_state).plugindata as *mut EffectState;
    (*state).reset();
    FMOD_OK
}

unsafe extern "C" fn shouldiprocess_callback(
    _dsp_state: *mut FMOD_DSP_STATE,
    inputs_idle: FMOD_BOOL,
    _length: std::os::raw::c_uint,
    _in_mask: FMOD_CHANNELMASK,
    _in_channels: c_int,
    _speaker_mode: FMOD_SPEAKERMODE,
) -> FMOD_RESULT {
    if inputs_idle != 0 {
        return FMOD_ERR_DSP_DONTPROCESS;
    }

    FMOD_OK
}

/// Processing is done here. Invoked by FMOD mixer.
/// See FMOD_DSP_PROCESS_CALLBACK docs.
unsafe extern "C" fn process_callback(
    dsp_state: *mut FMOD_DSP_STATE,
    length: std::os::raw::c_uint,
    in_buffer_array: *const FMOD_DSP_BUFFER_ARRAY,
    out_buffer_array: *mut FMOD_DSP_BUFFER_ARRAY,
    inputs_idle: FMOD_BOOL,
    op: FMOD_DSP_PROCESS_OPERATION,
) -> FMOD_RESULT {
    let state: *mut EffectState = (*dsp_state).plugindata as *mut EffectState;

    // todo list:
    // Calculate source coordinates
    // Calculate listener coordinates

    if op == FMOD_DSP_PROCESS_QUERY {
        if !in_buffer_array.is_null() && !out_buffer_array.is_null() {
            *(*out_buffer_array).buffernumchannels = *(*in_buffer_array).buffernumchannels;
            (*out_buffer_array).speakermode = (*in_buffer_array).speakermode;
        }

        if inputs_idle != 0 {
            // If the sound is idle, we still need to check the expected overall gain to help manage
            // channel counts. updateOverallGain won't do any processing - just determine how loud
            // the sound would be (according to attenuation, etc.) if it were playing.
            // todo: updateOverallGain(...)
            return FMOD_ERR_DSP_DONTPROCESS;
        }
    } else if op == FMOD_DSP_PROCESS_PERFORM {
        // updateOverallGain(...)

        let num_channels = *(*in_buffer_array).buffernumchannels as usize;
        let num_samples = num_channels * length as usize;

        let inbuf = slice::from_raw_parts(*(*in_buffer_array).buffers, num_samples);
        let outbuf = slice::from_raw_parts_mut(*(*out_buffer_array).buffers, num_samples);

        (*state).process(
            inbuf,
            outbuf,
            length as usize,
            *(*out_buffer_array).buffernumchannels as usize,
        );
    }

    FMOD_OK
}

unsafe extern "C" fn set_float_callback(
    dsp_state: *mut FMOD_DSP_STATE,
    index: c_int,
    value: c_float,
) -> FMOD_RESULT {
    let state: *mut EffectState = (*dsp_state).plugindata as *mut EffectState;
    let state = state.as_mut().unwrap();

    if index == 0 {
        state.set_gain(value);
        FMOD_OK
    } else {
        FMOD_ERR_INVALID_PARAM
    }
}

unsafe extern "C" fn get_float_callback(
    dsp_state: *mut FMOD_DSP_STATE,
    index: c_int,
    value: *mut c_float,
    _value_str: *mut c_char,
) -> FMOD_RESULT {
    let state: *mut EffectState = (*dsp_state).plugindata as *mut EffectState;
    let state = state.as_mut().unwrap();

    if index == 0 {
        *value = state.get_gain();
        FMOD_OK
    } else {
        FMOD_ERR_INVALID_PARAM
    }
}

static mut PARAM_GAIN: MaybeUninit<FMOD_DSP_PARAMETER_DESC> = MaybeUninit::uninit();

static mut PARAMETERS: [*mut FMOD_DSP_PARAMETER_DESC; 1] = [null_mut()];

// Fields that are empty/null will be assigned in FMODGetDSPDescription() if necessary.
static mut DSP_DESCRIPTION: FMOD_DSP_DESCRIPTION = FMOD_DSP_DESCRIPTION {
    pluginsdkversion: FMOD_PLUGIN_SDK_VERSION,
    name: [0; 32],
    version: 1,
    numinputbuffers: 1,
    numoutputbuffers: 1,
    create: Some(create_callback),
    release: Some(release_callback),
    reset: Some(reset_callback),
    read: None,
    process: Some(process_callback),
    setposition: None,
    numparameters: 1,
    paramdesc: null_mut(),
    setparameterfloat: Some(set_float_callback),
    setparameterint: None,
    setparameterbool: None,
    setparameterdata: None,
    getparameterfloat: Some(get_float_callback),
    getparameterint: None,
    getparameterbool: None,
    getparameterdata: None,
    shouldiprocess: Some(shouldiprocess_callback),
    userdata: null_mut(),
    sys_register: Some(sys_register_callback),
    sys_deregister: Some(sys_deregister_callback),
    sys_mix: None,
};

/// FMOD will call this function load the plugin defined by FMOD_DSP_DESCRIPTION.
/// See https://fmod.com/docs/2.02/api/white-papers-dsp-plugin-api.html#building-a-plug-in
#[no_mangle]
extern "C" fn FMODGetDSPDescription() -> *mut FMOD_DSP_DESCRIPTION {
    unsafe {
        let param_gain = PARAM_GAIN.as_mut_ptr();
        //todo make function to fill in the parameter fields.
        (*param_gain).type_ = FMOD_DSP_PARAMETER_TYPE_FLOAT;
        (*param_gain).name = str_to_c_char_array("Gain");
        (*param_gain).label = str_to_c_char_array("dB");
        static DESCRIPTION: &str = "Hello it's a description!\0";
        (*param_gain).description = DESCRIPTION.as_ptr() as *const c_char;
        (*param_gain).union = FMOD_DSP_PARAMETER_DESC_UNION {
            floatdesc: FMOD_DSP_PARAMETER_DESC_FLOAT {
                min: FMOD_GAIN_PARAM_GAIN_MIN,
                max: FMOD_GAIN_PARAM_GAIN_MAX,
                defaultval: FMOD_GAIN_PARAM_GAIN_DEFAULT,
                mapping: FMOD_DSP_PARAMETER_FLOAT_MAPPING::default(),
            },
        };

        DSP_DESCRIPTION.name = str_to_c_char_array("Phonon Spatializer");
        DSP_DESCRIPTION.paramdesc = PARAMETERS.as_mut_ptr();

        PARAMETERS[0] = param_gain;

        addr_of_mut!(DSP_DESCRIPTION)
    }
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

unsafe extern "C" fn sys_register_callback(_dsp_state: *mut FMOD_DSP_STATE) -> FMOD_RESULT {
    FMOD_OK
}

unsafe extern "C" fn sys_deregister_callback(_dsp_state: *mut FMOD_DSP_STATE) -> FMOD_RESULT {
    FMOD_OK
}
