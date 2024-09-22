use crate::fmod_state::FmodDspState;
use crate::parameter_spec::Params;
use crate::{EffectState, ParameterApplyType};
use libfmod::ffi::{
    FMOD_BOOL, FMOD_DSP_BUFFER_ARRAY, FMOD_DSP_PAN_3D_ROLLOFF_INVERSE,
    FMOD_DSP_PARAMETER_3DATTRIBUTES, FMOD_DSP_PARAMETER_ATTENUATION_RANGE,
    FMOD_DSP_PARAMETER_OVERALLGAIN, FMOD_DSP_PROCESS_OPERATION, FMOD_DSP_PROCESS_QUERY,
    FMOD_DSP_STATE, FMOD_ERR_DSP_DONTPROCESS, FMOD_ERR_INVALID_PARAM, FMOD_ERR_MEMORY, FMOD_OK,
    FMOD_RESULT, FMOD_SPEAKERMODE_STEREO,
};
use phonon::dsp::audio_buffer::{AudioBuffer, AudioSettings};
use phonon::dsp::speaker_layout::SpeakerLayoutType;
use phonon::effects::direct::{DirectEffect, TransmissionType};
use phonon::effects::panning::PanningEffect;
use phonon::simulators::direct::DirectSoundPath;
use std::os::raw::{c_char, c_float, c_int, c_uint, c_void};
use std::ptr::{null_mut, slice_from_raw_parts_mut};
use std::slice;

pub(crate) unsafe extern "C" fn create_callback(dsp_state: *mut FMOD_DSP_STATE) -> FMOD_RESULT {
    let dsp_state_wrapped = FmodDspState::new(dsp_state);
    let frame_size = dsp_state_wrapped.get_block_size().unwrap() as usize;
    let sampling_rate = dsp_state_wrapped.get_sample_rate().unwrap();

    let audio_settings = AudioSettings::new(sampling_rate, frame_size);
    // todo, support mono as well (don't forget the process callback)
    let speaker_layout = SpeakerLayoutType::Stereo;

    // why does distance attenuation range seem to exist twice?
    let fmod_gain_state = Box::new(EffectState {
        source: Default::default(),
        overall_gain: FMOD_DSP_PARAMETER_OVERALLGAIN {
            linear_gain: 1.0,
            linear_gain_additive: 0.0,
        },
        apply_distance_attenuation: ParameterApplyType::UserDefined,
        apply_air_absorption: ParameterApplyType::Disable,
        apply_directivity: ParameterApplyType::Disable,
        apply_occlusion: ParameterApplyType::Disable,
        apply_transmission: ParameterApplyType::Disable,
        distance_attenuation: 1.0,
        distance_attenuation_rolloff_type: FMOD_DSP_PAN_3D_ROLLOFF_INVERSE,
        distance_attenuation_min_distance: 1.0,
        distance_attenuation_max_distance: 20.0,
        direct_sound_path: Default::default(),
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
        audio_settings: audio_settings,
        in_buffer_stereo: AudioBuffer::new(frame_size),
        in_buffer_mono: AudioBuffer::new(frame_size),
        out_buffer: AudioBuffer::new(frame_size),
        direct_buffer: AudioBuffer::new(frame_size),
        mono_buffer: AudioBuffer::new(frame_size),
        panning_effect: PanningEffect::new(speaker_layout),
        direct_effect: DirectEffect::new(audio_settings),
    });

    let struct_ptr: *mut EffectState = Box::into_raw(fmod_gain_state);
    (*dsp_state).plugindata = struct_ptr as *mut c_void;

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

/// Processing is done here. Invoked by FMOD mixer.
/// See FMOD_DSP_PROCESS_CALLBACK docs.
pub(crate) unsafe extern "C" fn process_callback(
    dsp_state: *mut FMOD_DSP_STATE,
    length: c_uint,
    in_buffer_array: *const FMOD_DSP_BUFFER_ARRAY,
    out_buffer_array: *mut FMOD_DSP_BUFFER_ARRAY,
    inputs_idle: FMOD_BOOL,
    op: FMOD_DSP_PROCESS_OPERATION,
) -> FMOD_RESULT {
    let dsp_state = FmodDspState::new(dsp_state);
    let effect_state = dsp_state.get_effect_state();

    if op == FMOD_DSP_PROCESS_QUERY {
        if !out_buffer_array.is_null() {
            (*out_buffer_array).speakermode = FMOD_SPEAKERMODE_STEREO;
            *(*out_buffer_array).buffernumchannels = 2;
            *(*out_buffer_array).bufferchannelmask = 0;
        }

        if inputs_idle != 0 {
            // todo updateOverallGain
            return FMOD_ERR_DSP_DONTPROCESS;
        }
    } else {
        // todo updateOverallGain
        let num_channels = *(*in_buffer_array).buffernumchannels as usize;
        let num_samples = num_channels * length as usize;

        let new_block_size = dsp_state.get_block_size().unwrap() as usize;
        let new_sample_rate = dsp_state.get_sample_rate().unwrap();

        let block_size = (*effect_state).audio_settings.frame_size;
        let sample_rate = (*effect_state).audio_settings.sampling_rate;

        if (new_block_size != block_size) || (new_sample_rate != sample_rate) {
            // todo: I haven't found a way to test this path yet
            let audio_settings = AudioSettings::new(new_sample_rate, new_block_size);
            (*effect_state).in_buffer_stereo = AudioBuffer::new(new_block_size);
            (*effect_state).in_buffer_mono = AudioBuffer::new(new_block_size);
            (*effect_state).out_buffer = AudioBuffer::new(new_block_size);
            (*effect_state).direct_buffer = AudioBuffer::new(new_block_size);
            (*effect_state).mono_buffer = AudioBuffer::new(new_block_size);
            (*effect_state).direct_effect = DirectEffect::new(audio_settings);
            (*effect_state).audio_settings = audio_settings;
        }

        let inbuf = slice::from_raw_parts(*(*in_buffer_array).buffers, num_samples);
        let outbuf = slice::from_raw_parts_mut(*(*out_buffer_array).buffers, num_samples);
        (*effect_state).process(
            inbuf,
            outbuf,
            length as usize,
            *(*out_buffer_array).buffernumchannels as usize,
        );
    }

    FMOD_OK
}

// todo: Check if all set and get callbacks return FMOD_ERR_INVALID_PARAM when the index is unknown

#[expect(dead_code, reason = "No float params have been added yet")]
pub(crate) unsafe extern "C" fn set_float_callback(
    dsp_state: *mut FMOD_DSP_STATE,
    _index: c_int,
    _value: c_float,
) -> FMOD_RESULT {
    let state: *mut EffectState = (*dsp_state).plugindata as *mut EffectState;
    let _state = state.as_mut().unwrap();

    FMOD_OK
}

#[expect(dead_code, reason = "No float params have been added yet")]
pub(crate) unsafe extern "C" fn get_float_callback(
    dsp_state: *mut FMOD_DSP_STATE,
    _index: c_int,
    _value: *mut c_float,
    _value_str: *mut c_char,
) -> FMOD_RESULT {
    let state: *mut EffectState = (*dsp_state).plugindata as *mut EffectState;
    let _state = state.as_mut().unwrap();

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

    if let Some(param) = Params::from_repr(index) {
        match param {
            // todo get rid of duplicated code
            Params::SourcePosition => {
                let source_ptr =
                    (&mut state.source) as *mut FMOD_DSP_PARAMETER_3DATTRIBUTES as *mut u8;

                let data_slice = slice::from_raw_parts(data as *const u8, length as usize);
                let dest_slice = slice_from_raw_parts_mut(source_ptr, length as usize);

                dest_slice.as_mut().unwrap().copy_from_slice(data_slice);
            }
            Params::DirectSoundPath => {
                let source_ptr = (&mut state.direct_sound_path) as *mut DirectSoundPath as *mut u8;

                let data_slice = slice::from_raw_parts(data as *const u8, length as usize);
                let dest_slice = slice_from_raw_parts_mut(source_ptr, length as usize);

                dest_slice.as_mut().unwrap().copy_from_slice(data_slice);
            }
            _ => return FMOD_ERR_INVALID_PARAM,
        }
    } else {
        return FMOD_OK;
    }

    FMOD_OK
}

pub(crate) unsafe extern "C" fn get_data_callback(
    dsp_state: *mut FMOD_DSP_STATE,
    _index: c_int,
    _data: *mut *mut c_void,
    _length: *mut c_uint,
    _valuestr: *mut c_char,
) -> FMOD_RESULT {
    let dsp_state_wrapped = FmodDspState::new(dsp_state);
    let _effect = dsp_state_wrapped.get_effect_state();

    FMOD_OK
}

pub(crate) unsafe extern "C" fn set_int_callback(
    dsp_state: *mut FMOD_DSP_STATE,
    index: c_int,
    value: c_int,
) -> FMOD_RESULT {
    let data = &mut *((*dsp_state).plugindata as *mut EffectState);

    if let Some(param) = Params::from_repr(index) {
        match param {
            Params::ApplyDistanceAttenuation => {
                data.apply_distance_attenuation = value.into();
            }
            Params::ApplyAirabsorption => data.apply_air_absorption = value.into(),
            Params::ApplyDirectivity => data.apply_directivity = value.into(),
            Params::ApplyOcclusion => data.apply_occlusion = value.into(),
            Params::ApplyTransmission => data.apply_transmission = value.into(),
            _ => return FMOD_OK, // todo should be FMOD_ERR_INVALID_PARAM,
        }
    } else {
        return FMOD_OK;
    }

    FMOD_OK
}

pub(crate) unsafe extern "C" fn get_int_callback(
    dsp_state: *mut FMOD_DSP_STATE,
    index: c_int,
    value: *mut c_int,
    _valuestr: *mut c_char,
) -> FMOD_RESULT {
    let dsp_state_wrapped = FmodDspState::new(dsp_state);
    let effect = dsp_state_wrapped.get_effect_state();

    if let Some(param) = Params::from_repr(index) {
        match param {
            Params::ApplyDistanceAttenuation => {
                let apply: c_int = (*effect).apply_distance_attenuation.into();
                value.write(apply);
            }
            Params::ApplyAirabsorption => {
                let apply: c_int = (*effect).apply_air_absorption.into();
                value.write(apply);
            }
            Params::ApplyDirectivity => {
                let apply: c_int = (*effect).apply_directivity.into();
                value.write(apply);
            }
            Params::ApplyOcclusion => {
                let apply: c_int = (*effect).apply_occlusion.into();
                value.write(apply);
            }
            _ => return FMOD_OK, // todo should be FMOD_ERR_INVALID_PARAM
        }
    } else {
        return FMOD_OK;
    }

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
