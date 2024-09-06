use crate::fmod_state::FmodDspState;
use crate::{EffectState, ParameterApplyType};
use libfmod::ffi::{
    FMOD_BOOL, FMOD_DSP_BUFFER_ARRAY, FMOD_DSP_PAN_3D_ROLLOFF_INVERSE,
    FMOD_DSP_PARAMETER_3DATTRIBUTES, FMOD_DSP_PARAMETER_ATTENUATION_RANGE,
    FMOD_DSP_PARAMETER_OVERALLGAIN, FMOD_DSP_PROCESS_OPERATION, FMOD_DSP_PROCESS_QUERY,
    FMOD_DSP_STATE, FMOD_ERR_DSP_DONTPROCESS, FMOD_ERR_INVALID_PARAM, FMOD_ERR_MEMORY, FMOD_OK,
    FMOD_RESULT, FMOD_SPEAKERMODE_STEREO,
};
use phonon::audio_buffer::{AudioBuffer, AudioSettings};
use phonon::direct_effect::{DirectEffect, TransmissionType};
use phonon::panning_effect::PanningEffect;
use phonon::speaker_layout::SpeakerLayoutType;
use std::os::raw::{c_char, c_float, c_int, c_uint, c_void};
use std::ptr::{null_mut, slice_from_raw_parts_mut};
use std::slice;
use strum::FromRepr;

// todo: This should be somewhere else. And there might be a rustier way to do this
#[derive(FromRepr, Debug, PartialEq)]
#[repr(i32)]
enum Params {
    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_DATA`
     *
     *  World-space position of the source. Automatically written by FMOD Studio.
     */
    SourcePosition = 0,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_DATA`
     *
     *  Overall linear gain of this effect. Automatically read by FMOD Studio.
     */
    OverallGain = 1,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_INT`
     *
     *  **Range**: 0 to 2.
     *
     *  How to render distance attenuation.
     *
     *  -   `0`: Don't render distance attenuation.
     *  -   `1`: Use a distance attenuation value calculated using the default physics-based model.
     *  -   `2`: Use a distance attenuation value calculated using the curve specified in the FMOD Studio UI.
     */
    ApplyDistanceAttenuation = 2,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_INT`
     *
     *  **Range**: 0 to 2.
     *
     *  How to render air absorption.
     *
     *  -   `0`: Don't render air absorption.
     *  -   `1`: Use air absorption values calculated using the default exponential decay model.
     *  -   `2`: Use air absorption values specified in the \c AirabsorptionLow, \c AIRABSORPTION_MID, and
     *           \c AIRABSORPTION_HIGH parameters.
     */
    ApplyAirabsorption,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_INT`
     *
     *  **Range**: 0 to 2.
     *
     *  How to render directivity.
     *
     *  -   `0`: Don't render directivity.
     *  -   `1`: Use a directivity value calculated using the default dipole model, driven by the
     *           \c DirectivityDipoleweight and \c DIRECTIVITY_DIPOLEPOWER parameters.
     *  -   `2`: Use the directivity value specified in the \c DIRECTIVITY parameter.
     */
    ApplyDirectivity,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_INT`
     *
     *  **Range**: 0 to 2.
     *
     *  How to render occlusion.
     *
     *  -   `0`: Don't render occlusion.
     *  -   `1`: Use the occlusion value calculated by the game engine using simulation, and provided via the
     *           \c SimulationOutputs parameter.
     *  -   `2`: Use the occlusion value specified in the \c OCCLUSION parameter.
     */
    ApplyOcclusion,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_INT`
     *
     *  **Range**: 0 to 2.
     *
     *  How to render transmission.
     *
     *  -   `0`: Don't render transmission.
     *  -   `1`: Use the transmission values calculated by the game engine using simulation, and provided via the
     *           \c SimulationOutputs parameter.
     *  -   `2`: Use the transmission values specified in the \c TransmissionLow, \c TRANSMISSION_MID, and
     *           \c TransmissionHigh parameters.
     */
    ApplyTransmission,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_BOOL`
     *
     *  If true, reflections are rendered, using the data calculated by the game engine using simulation, and provided
     *  via the \c SimulationOutputs parameter.
     */
    ApplyReflections,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_BOOL`
     *
     *  If true, pathing is rendered, using the data calculated by the game engine using simulation, and provided
     *  via the \c SimulationOutputs parameter.
     */
    ApplyPathing,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_INT`
     *
     *  **Range**: 0 to 1.
     *
     *  Controls how HRTFs are interpolated when the source moves relative to the listener.
     *
     *  - `0`: Nearest-neighbor interpolation.
     *  - `1`: Bilinear interpolation.
     */
    HrtfInterpolation,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_FLOAT`
     *
     *  Not currently used.
     */
    DistanceAttenuation,

    /*
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_INT`
     *
     *  **Range**: 0 to 4.
     *
     *  Type of distance attenuation curve preset to use when \c APPLY_DISTANCEATTENUATION is \c 1.
     *
     *  - `0`: Linear squared rolloff.
     *  - `1`: Linear rolloff.
     *  - `2`: Inverse rolloff.
     *  - `3`: Inverse squared rolloff.
     *  - `4`: Custom rolloff.
     */
    DistanceAttenuationRolloffType,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_FLOAT`
     *
     *  **Range**: 0 to 10000.
     *
     *  Minimum distance value for the distance attenuation curve.
     */
    DistanceAttenuationMinDistance,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_FLOAT`
     *
     *  **Range**: 0 to 10000.
     *
     *  Maximum distance value for the distance attenuation curve.
     */
    DistanceAttenuationMaxDistance,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_FLOAT`
     *
     *  **Range**: 0 to 1.
     *
     *  The low frequency (up to 800 Hz) EQ value for air absorption. Only used if \c ApplyAirabsorption is set to
     *  \c 2. 0 = low frequencies are completely attenuated, 1 = low frequencies are not attenuated at all.
     */
    AirAbsorptionLow,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_FLOAT`
     *
     *  **Range**: 0 to 1.
     *
     *  The middle frequency (800 Hz - 8 kHz) EQ value for air absorption. Only used if \c ApplyAirabsorption is set
     *  to \c 2. 0 = middle frequencies are completely attenuated, 1 = middle frequencies are not attenuated at all.
     */
    AirAbsorptionMid,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_FLOAT`
     *
     *  **Range**: 0 to 1.
     *
     *  The high frequency (8 kHz and above) EQ value for air absorption. Only used if \c ApplyAirabsorption is set to
     *  \c 2. 0 = high frequencies are completely attenuated, 1 = high frequencies are not attenuated at all.
     */
    AirAbsorptionHigh,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_FLOAT`
     *
     *  **Range**: 0 to 1.
     *
     *  The directivity attenuation value. Only used if \c ApplyDirectivity is set to \c 2. 0 = sound is completely
     *  attenuated, 1 = sound is not attenuated at all.
     */
    Directivity,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_FLOAT`
     *
     *  **Range**: 0 to 1.
     *
     *  Blends between monopole (omnidirectional) and dipole directivity patterns. 0 = pure monopole (sound is emitted
     *  in all directions with equal intensity), 1 = pure dipole (sound is focused to the front and back of the source).
     *  At 0.5, the source has a cardioid directivity, with most of the sound emitted to the front of the source. Only
     *  used if \c ApplyDirectivity is set to \c 1.
     */
    DirectivityDipoleWeight,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_FLOAT`
     *
     *  **Range**: 0 to 4.
     *
     *  Controls how focused the dipole directivity is. Higher values result in sharper directivity patterns. Only used
     *  if \c ApplyDirectivity is set to \c 1.
     */
    DirectivityDipolePower,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_FLOAT`
     *
     *  **Range**: 0 to 1.
     *
     *  The occlusion attenuation value. Only used if \c ApplyOcclusion is set to \c 2. 0 = sound is completely
     *  attenuated, 1 = sound is not attenuated at all.
     */
    OCCLUSION,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_INT`
     *
     *  **Range**: 0 to 1.
     *
     *  Specifies how the transmission filter is applied.
     *
     * - `0`: Transmission is modeled as a single attenuation factor.
     * - `1`: Transmission is modeled as a 3-band EQ.
     */
    TransmissionType,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_FLOAT`
     *
     *  **Range**: 0 to 1.
     *
     *  The low frequency (up to 800 Hz) EQ value for transmission. Only used if \c ApplyTransmission is set to \c 2.
     *  0 = low frequencies are completely attenuated, 1 = low frequencies are not attenuated at all.
     */
    TransmissionLow,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_FLOAT`
     *
     *  **Range**: 0 to 1.
     *
     *  The middle frequency (800 Hz to 8 kHz) EQ value for transmission. Only used if \c ApplyTransmission is set to
     *  \c 2. 0 = middle frequencies are completely attenuated, 1 = middle frequencies are not attenuated at all.
     */
    TRANSMISSION_MID,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_FLOAT`
     *
     *  **Range**: 0 to 1.
     *
     *  The high frequency (8 kHz and above) EQ value for transmission. Only used if \c ApplyTransmission is set to
     *  \c 2. 0 = high frequencies are completely attenuated, 1 = high frequencies are not attenuated at all.
     */
    TransmissionHigh,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_FLOAT`
     *
     *  **Range**: 0 to 1.
     *
     *  The contribution of the direct sound path to the overall mix for this event. Lower values reduce the
     *  contribution more.
     */
    DirectMixLevel,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_BOOL`
     *
     *  If true, applies HRTF-based 3D audio rendering to reflections. Results in an improvement in spatialization
     *  quality when using convolution or hybrid reverb, at the cost of slightly increased CPU usage.
     */
    ReflectionsBinaural,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_FLOAT`
     *
     *  **Range**: 0 to 10.
     *
     *  The contribution of reflections to the overall mix for this event. Lower values reduce the contribution more.
     */
    ReflectionsMixLevel,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_BOOL`
     *
     *  If true, applies HRTF-based 3D audio rendering to pathing. Results in an improvement in spatialization
     *  quality, at the cost of slightly increased CPU usage.
     */
    PathingBinaural,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_FLOAT`
     *
     *  **Range**: 0 to 10.
     *
     *  The contribution of pathing to the overall mix for this event. Lower values reduce the contribution more.
     */
    PathingMixLevel,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_DATA`
     *
     *  **DEPRECATED**
     *
     *  Pointer to the `IPLSimulationOutputs` structure containing simulation results.
     */
    SimulationOutputs,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_BOOL`
     *
     *  If true, applies HRTF-based 3D audio rendering to the direct sound path. Otherwise, sound is panned based on
     *  the speaker configuration.
     */
    DirectBinaural,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_DATA`
     *
     *  (FMOD Studio 2.02+) The event's min/max distance range. Automatically set by FMOD Studio.
     */
    DistanceAttenuationRange,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_INT`
     *
     *  Handle of the `IPLSource` object to use for obtaining simulation results. The handle can
     *  be obtained by calling `iplFMODAddSource`.
     */
    SimulationOutputsHandle,

    /**
     *  **Type**: `FMOD_DSP_PARAMETER_TYPE_INT`
     *
     *  **Range**: 0 to 2.
     *
     *  Controls the output format.
     *
     *  - `0`: Output will be the format in FMOD's mixer.
     *  - `1`: Output will be the format from FMOD's final output.
     *  - `2`: Output will be the format from the event's input.
     */
    OutputFormat,
}

pub(crate) unsafe extern "C" fn create_callback(dsp_state: *mut FMOD_DSP_STATE) -> FMOD_RESULT {
    // todo: I guess the settings frame_size, sampling_rate and speaker_layout could change at
    // any time in the other callbacks.

    let dsp_state_wrapped = FmodDspState::new(dsp_state);
    //dsp_state_wrapped.log_message("testMessage");
    let frame_size = dsp_state_wrapped.get_block_size().unwrap() as usize;
    let sampling_rate = dsp_state_wrapped.get_sample_rate().unwrap();

    let audio_settings = AudioSettings::new(sampling_rate, frame_size);
    let speaker_layout = SpeakerLayoutType::Stereo; // todo, support mono as well

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
    length: std::os::raw::c_uint,
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

        let block_size = dsp_state.get_block_size().unwrap();
        let sample_rate = dsp_state.get_sample_rate().unwrap();

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
    _index: c_int,
    _value: *mut c_float,
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

    if let Some(param) = Params::from_repr(index) {
        match param {
            Params::SourcePosition => {
                let source_ptr =
                    (&mut state.source) as *mut FMOD_DSP_PARAMETER_3DATTRIBUTES as *mut u8;

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
    index: c_int,
    data: *mut *mut c_void,
    length: *mut c_uint,
    _valuestr: *mut c_char,
) -> FMOD_RESULT {
    let dsp_state_wrapped = FmodDspState::new(dsp_state);
    let effect = dsp_state_wrapped.get_effect_state();

    FMOD_OK
}

pub(crate) unsafe extern "C" fn set_int_callback(
    dsp_state: *mut FMOD_DSP_STATE,
    index: c_int,
    value: c_int,
) -> FMOD_RESULT {
    let dsp_state_wrapped = FmodDspState::new(dsp_state);
    let effect = dsp_state_wrapped.get_effect_state();

    if let Some(param) = Params::from_repr(index) {
        match param {
            Params::ApplyDistanceAttenuation => {
                (*effect).apply_distance_attenuation = value.into();
            }
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
    valuestr: *mut c_char,
) -> FMOD_RESULT {
    let dsp_state_wrapped = FmodDspState::new(dsp_state);
    let effect = dsp_state_wrapped.get_effect_state();
    let apply_da: c_int = (*effect).apply_distance_attenuation.into();

    if let Some(param) = Params::from_repr(index) {
        match param {
            Params::ApplyDistanceAttenuation => {
                *value = apply_da;
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
