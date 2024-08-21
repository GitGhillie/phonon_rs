use crate::{EffectState, ParameterApplyType};
use libfmod::ffi::{
    FMOD_BOOL, FMOD_CHANNELMASK, FMOD_DSP_BUFFER_ARRAY, FMOD_DSP_PROCESS_OPERATION,
    FMOD_DSP_PROCESS_QUERY, FMOD_DSP_STATE, FMOD_ERR_DSP_DONTPROCESS, FMOD_ERR_INVALID_PARAM,
    FMOD_ERR_MEMORY, FMOD_OK, FMOD_RESULT, FMOD_SPEAKERMODE,
};
use std::os::raw::{c_char, c_float, c_int};
use std::ptr::null_mut;
use std::slice;
use phonon::audio_buffer::AudioBuffer;
use phonon::direct_effect::{DirectEffect, TransmissionType};
use phonon::panning_effect::PanningEffect;
// Todo: These callbacks should probably not be pub like this

pub(crate) unsafe extern "C" fn create_callback(dsp_state: *mut FMOD_DSP_STATE) -> FMOD_RESULT {
    let num_samples = 1024; //todo

    let fmod_gain_state = Box::new(EffectState {
        source: Default::default(),
        overall_gain: Default::default(),
        apply_distance_attenuation: ParameterApplyType::SimulationDefined,
        apply_air_absorption: ParameterApplyType::Disable,
        apply_directivity: ParameterApplyType::Disable,
        apply_occlusion: ParameterApplyType::Disable,
        apply_transmission: ParameterApplyType::Disable,
        distance_attenuation: 0.0,
        distance_attenuation_rolloff_type: 0,
        distance_attenuation_min_distance: 0.0,
        distance_attenuation_max_distance: 0.0,
        air_absorption: [0.0, 0.0, 0.0],
        directivity: 0.0,
        dipole_weight: 0.0,
        dipole_power: 0.0,
        occlusion: 0.0,
        transmission_type: TransmissionType::FrequencyIndependent,
        transmission: [0.0, 0.0, 0.0],
        attenuation_range: Default::default(),
        attenuation_range_set: false,
        in_buffer_stereo: AudioBuffer::new(num_samples),
        in_buffer_mono: AudioBuffer::new(num_samples),
        out_buffer: AudioBuffer::new(num_samples),
        direct_buffer: AudioBuffer::new(num_samples),
        mono_buffer: AudioBuffer::new(num_samples),
        panning_effect: PanningEffect,
        direct_effect: DirectEffect,
    });

    let struct_ptr: *mut EffectState = Box::into_raw(fmod_gain_state);
    (*dsp_state).plugindata = struct_ptr as *mut std::os::raw::c_void;

    if (*dsp_state).plugindata.is_null() {
        return FMOD_ERR_MEMORY;
    }

    FMOD_OK
}

pub(crate) unsafe extern "C" fn release_callback(dsp_state: *mut FMOD_DSP_STATE) -> FMOD_RESULT {
    let struct_ptr: *mut EffectState = (*dsp_state).plugindata as *mut EffectState;
    drop(Box::from_raw(struct_ptr));
    (*dsp_state).plugindata = null_mut();
    FMOD_OK
}

pub(crate) unsafe extern "C" fn reset_callback(dsp_state: *mut FMOD_DSP_STATE) -> FMOD_RESULT {
    let state: *mut EffectState = (*dsp_state).plugindata as *mut EffectState;
    (*state).reset();
    FMOD_OK
}

pub(crate) unsafe extern "C" fn shouldiprocess_callback(
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
pub(crate) unsafe extern "C" fn process_callback(
    dsp_state: *mut FMOD_DSP_STATE,
    length: std::os::raw::c_uint,
    in_buffer_array: *const FMOD_DSP_BUFFER_ARRAY,
    out_buffer_array: *mut FMOD_DSP_BUFFER_ARRAY,
    inputs_idle: FMOD_BOOL,
    op: FMOD_DSP_PROCESS_OPERATION,
) -> FMOD_RESULT {
    let state: *mut EffectState = (*dsp_state).plugindata as *mut EffectState;

    if op == FMOD_DSP_PROCESS_QUERY {
        if !in_buffer_array.is_null() && !out_buffer_array.is_null() {
            *(*out_buffer_array).buffernumchannels = *(*in_buffer_array).buffernumchannels;
            (*out_buffer_array).speakermode = (*in_buffer_array).speakermode;
        }

        if inputs_idle != 0 {
            return FMOD_ERR_DSP_DONTPROCESS;
        }
    } else {
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

pub(crate) unsafe extern "C" fn set_float_callback(
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

pub(crate) unsafe extern "C" fn get_float_callback(
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

pub(crate) unsafe extern "C" fn sys_register_callback(
    _dsp_state: *mut FMOD_DSP_STATE,
) -> FMOD_RESULT {
    FMOD_OK
}

pub(crate) unsafe extern "C" fn sys_deregister_callback(
    _dsp_state: *mut FMOD_DSP_STATE,
) -> FMOD_RESULT {
    FMOD_OK
}
