use crate::{EffectState, ParameterApplyType};
use libfmod::ffi::{
    FMOD_BOOL, FMOD_CHANNELMASK, FMOD_DSP_BUFFER_ARRAY, FMOD_DSP_PAN_3D_ROLLOFF_INVERSE,
    FMOD_DSP_PARAMETER_3DATTRIBUTES, FMOD_DSP_PARAMETER_ATTENUATION_RANGE,
    FMOD_DSP_PROCESS_OPERATION, FMOD_DSP_PROCESS_QUERY, FMOD_DSP_STATE, FMOD_ERR_DSP_DONTPROCESS,
    FMOD_ERR_INVALID_PARAM, FMOD_ERR_MEMORY, FMOD_OK, FMOD_RESULT, FMOD_SPEAKERMODE,
};
use phonon::audio_buffer::{AudioBuffer, AudioSettings};
use phonon::direct_effect::{DirectEffect, TransmissionType};
use phonon::panning_effect::PanningEffect;
use phonon::speaker_layout::SpeakerLayoutType;
use std::os::raw::{c_char, c_float, c_int, c_uint, c_void};
use std::ptr::{null_mut, slice_from_raw_parts_mut};
use std::slice;

// todo: This should be somewhere else. And there might be a rustier way to do this
enum Params {
    SourcePosition = 1,
    OverallGain,
    ApplyDistanceAttenuation,
}

pub(crate) unsafe extern "C" fn create_callback(dsp_state: *mut FMOD_DSP_STATE) -> FMOD_RESULT {
    // todo: I guess the settings frame_size, sampling_rate and speaker_layout could change at
    // any time in the other callbacks.
    let frame_size = 1024; //todo
    let sampling_rate = 48_000; //todo

    //println!("TODO remove this");

    let audio_settings = AudioSettings::new(sampling_rate, frame_size);

    let speaker_layout = SpeakerLayoutType::Stereo; // todo, support mono as well

    // why does distance attenuation range seem to exist twice?
    let fmod_gain_state = Box::new(EffectState {
        source: Default::default(),
        overall_gain: Default::default(),
        apply_distance_attenuation: ParameterApplyType::UserDefine,
        apply_air_absorption: ParameterApplyType::Disable,
        apply_directivity: ParameterApplyType::Disable,
        apply_occlusion: ParameterApplyType::Disable,
        apply_transmission: ParameterApplyType::Disable,
        distance_attenuation: 1.0,
        distance_attenuation_rolloff_type: FMOD_DSP_PAN_3D_ROLLOFF_INVERSE,
        distance_attenuation_min_distance: 1.0,
        distance_attenuation_max_distance: 20.0,
        air_absorption: [1.0, 1.0, 1.0],
        directivity: 1.0,
        dipole_weight: 0.0,
        dipole_power: 1.0,
        occlusion: 1.0,
        transmission_type: TransmissionType::FrequencyIndependent,
        transmission: [1.0, 1.0, 1.0],
        attenuation_range: FMOD_DSP_PARAMETER_ATTENUATION_RANGE {
            min: 1.0,
            max: 20.0,
        },
        attenuation_range_set: false,
        in_buffer_stereo: AudioBuffer::new(frame_size),
        in_buffer_mono: AudioBuffer::new(frame_size),
        out_buffer: AudioBuffer::new(frame_size),
        direct_buffer: AudioBuffer::new(frame_size),
        mono_buffer: AudioBuffer::new(frame_size),
        panning_effect: PanningEffect::new(speaker_layout),
        direct_effect: DirectEffect::new(audio_settings),
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

    FMOD_OK
}

pub(crate) unsafe extern "C" fn get_float_callback(
    dsp_state: *mut FMOD_DSP_STATE,
    index: c_int,
    value: *mut c_float,
    _value_str: *mut c_char,
) -> FMOD_RESULT {
    let state: *mut EffectState = (*dsp_state).plugindata as *mut EffectState;
    let state = state.as_mut().unwrap();

    FMOD_OK
}

pub(crate) unsafe extern "C" fn set_data_callback(
    dsp_state: *mut FMOD_DSP_STATE,
    index: c_int,
    data: *mut c_void,
    length: c_uint,
) -> FMOD_RESULT {
    let state: *mut EffectState = (*dsp_state).plugindata as *mut EffectState;
    let state = state.as_mut().unwrap();

    // todo replace hardcoded match values
    match index {
        0 => {
            // SOURCE_POSITION
            let source_ptr = (&mut state.source) as *mut FMOD_DSP_PARAMETER_3DATTRIBUTES as *mut u8;

            let data_slice = slice::from_raw_parts(data as *const u8, length as usize);
            let mut dest_slice = slice_from_raw_parts_mut(source_ptr, length as usize);

            dest_slice.as_mut().unwrap().copy_from_slice(data_slice);
        }
        _ => return FMOD_ERR_INVALID_PARAM,
    }

    FMOD_OK
}

pub(crate) unsafe extern "C" fn get_data_callback(
    dsp_state: *mut FMOD_DSP_STATE,
    index: c_int,
    data: *mut *mut c_void,
    length: *mut c_uint,
    valuestr: *mut c_char,
) -> FMOD_RESULT {
    FMOD_OK
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
