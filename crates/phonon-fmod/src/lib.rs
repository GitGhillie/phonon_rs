//! FMOD Plugin for the phonon crate.
//!
//! On the FMOD Studio side the plugin needs to be built as a dynamic library
//! and loaded by placing it in one of the folders indicated here:
//! https://www.fmod.com/docs/2.02/studio/plugin-reference.html#loading-plug-ins
//!
//! On the application side the plugin can either be dynamically or statically linked.
//! By default, this should be done statically.

// See example https://github.com/lebedec/libfmod/blob/main/libfmod/tests/examples/core.rs
// todo remove this

use std::ffi::CString;
use libfmod::ffi::{FMOD_BOOL, FMOD_CHANNELMASK, FMOD_DSP_BUFFER_ARRAY, FMOD_DSP_DESCRIPTION, FMOD_DSP_PARAMETER_DESC, FMOD_DSP_PARAMETER_DESC_FLOAT, FMOD_DSP_PARAMETER_DESC_UNION, FMOD_DSP_PARAMETER_FLOAT_MAPPING, FMOD_DSP_PARAMETER_TYPE, FMOD_DSP_PARAMETER_TYPE_FLOAT, FMOD_DSP_PROCESS_OPERATION, FMOD_DSP_PROCESS_QUERY, FMOD_DSP_STATE, FMOD_ERR_DSP_DONTPROCESS, FMOD_ERR_MEMORY, FMOD_OK, FMOD_PLUGIN_SDK_VERSION, FMOD_RESULT, FMOD_SPEAKERMODE};
use std::os::raw::{c_char, c_float, c_int};
use std::ptr::null_mut;
use std::ptr::addr_of_mut;
use std::slice;
use libfmod::{DspDescription, DspParameterDesc};

const FMOD_GAIN_PARAM_GAIN_MIN: f32 = -80.0;
const FMOD_GAIN_PARAM_GAIN_MAX: f32 = 10.0;
const FMOD_GAIN_PARAM_GAIN_DEFAULT: f32 = 0.0;
const FMOD_GAIN_RAMP_COUNT: i32 = 256;

fn db_to_linear(db_value: f32) -> f32 {
    if db_value <= FMOD_GAIN_PARAM_GAIN_MIN {
        0.0
    } else {
        10.0_f32.powf(db_value / 20.0)
    }
}

struct FmodGainState {
    target_gain: f32,
    current_gain: f32,
    ramp_samples_left: i32,
    //invert: bool,
}

impl FmodGainState {
    fn new() -> Self {
        let gain = db_to_linear(FMOD_GAIN_PARAM_GAIN_DEFAULT);

        Self {
            target_gain: gain,
            current_gain: gain,
            ramp_samples_left: 0,
            //invert: false,
        }
    }

    fn reset(&mut self) {
        self.current_gain = self.target_gain;
        self.ramp_samples_left = 0;
    }

    fn set_gain(&mut self, gain: f32) {
        self.target_gain = db_to_linear(gain);
        self.ramp_samples_left = FMOD_GAIN_RAMP_COUNT;
    }

    fn process(
        &mut self,
        in_buffer: &[f32],
        out_buffer: &mut [f32],
        length: usize,
        channels: usize,
    ) {
        let mut gain = self.current_gain;
        let mut len = length;

        let mut i = 0;

        if self.ramp_samples_left > 0 {
            let target = self.target_gain;
            let delta = (target - gain) / self.ramp_samples_left as f32;

            while len > 0 {
                if self.ramp_samples_left > 0 {
                    self.ramp_samples_left -= 1;
                    gain += delta;
                    for _ in 0..channels {
                        out_buffer[i] = in_buffer[i] * gain;
                        i += 1;
                    }
                } else {
                    gain = target;
                    break;
                }

                len -= 1;
            }
        }

        let mut samples = len * channels;
        while samples > 0 {
            samples -= 1;
            out_buffer[i] = in_buffer[i] * gain;
            i += 1;
        }

        self.current_gain = gain;
    }
}

static mut FMOD_GAIN_STATE: FmodGainState = FmodGainState {
    target_gain: 1.0,
    current_gain: 1.0,
    ramp_samples_left: 0,
};

unsafe extern "C" fn create_callback(dsp_state: *mut FMOD_DSP_STATE) -> FMOD_RESULT {
    let struct_ptr: *mut FmodGainState = addr_of_mut!(FMOD_GAIN_STATE);
    (*dsp_state).plugindata = struct_ptr as *mut std::os::raw::c_void;

    if (*dsp_state).plugindata.is_null() {
        return FMOD_ERR_MEMORY;
    }

    FMOD_OK
}

unsafe extern "C" fn release_callback(dsp_state: *mut FMOD_DSP_STATE) -> FMOD_RESULT {
    (*dsp_state).plugindata = std::ptr::null_mut();
    FMOD_OK
}

unsafe extern "C" fn reset_callback(dsp_state: *mut FMOD_DSP_STATE) -> FMOD_RESULT {
    let state: *mut FmodGainState = (*dsp_state).plugindata as *mut FmodGainState;
    (*state).reset();
    FMOD_OK
}

unsafe extern "C" fn shouldiprocess_callback(
    dsp_state: *mut FMOD_DSP_STATE,
    inputs_idle: FMOD_BOOL,
    length: std::os::raw::c_uint,
    in_mask: FMOD_CHANNELMASK,
    in_channels: std::os::raw::c_int,
    speaker_mode: FMOD_SPEAKERMODE,
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
    let state: *mut FmodGainState = (*dsp_state).plugindata as *mut FmodGainState;

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

static mut PARAM_GAIN: FMOD_DSP_PARAMETER_DESC = FMOD_DSP_PARAMETER_DESC {
    type_: FMOD_DSP_PARAMETER_TYPE_FLOAT,
    name: [0; 16],
    label: [0; 16],
    description: null_mut(),
    union: unsafe { std::mem::zeroed() },
};

static mut PARAMETERS: [*mut FMOD_DSP_PARAMETER_DESC; 1] = [&mut unsafe { PARAM_GAIN }];

// static mut DSP_DESCRIPTION: DspDescription = DspDescription {
//     pluginsdkversion: FMOD_PLUGIN_SDK_VERSION,
//     name: [0; 32],
//     version: 0,
//     numinputbuffers: 1,
//     numoutputbuffers: 1,
//     create: Some(create_callback),
//     release: Some(release_callback),
//     reset: Some(reset_callback),
//     read: None,
//     process: Some(process_callback),
//     setposition: None,
//     paramdesc: vec![],
//     setparameterfloat: None,
//     setparameterint: None,
//     setparameterbool: None,
//     setparameterdata: None,
//     getparameterfloat: None,
//     getparameterint: None,
//     getparameterbool: None,
//     getparameterdata: None,
//     shouldiprocess: None,
//     userdata: null_mut(),
//     sys_register: None,
//     sys_deregister: None,
//     sys_mix: None,
// };

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
    numparameters: 0,
    paramdesc: null_mut(),
    setparameterfloat: None,
    setparameterint: None,
    setparameterbool: None,
    setparameterdata: None,
    getparameterfloat: None,
    getparameterint: None,
    getparameterbool: None,
    getparameterdata: None,
    shouldiprocess: Some(shouldiprocess_callback),
    userdata: null_mut(),
    sys_register: None,
    sys_deregister: None,
    sys_mix: None,
};

/// FMOD will call this function load the plugin defined by FMOD_DSP_DESCRIPTION.
/// See https://fmod.com/docs/2.02/api/white-papers-dsp-plugin-api.html#building-a-plug-in
#[no_mangle]
extern "C" fn FMODGetDSPDescription() -> *mut FMOD_DSP_DESCRIPTION {
    unsafe {
        // PARAM_GAIN.name = str_to_c_char_array("Gain");
        // PARAM_GAIN.label = str_to_c_char_array("dB");
        // static DESCRIPTION: &str = "Hello it's a description!\0";
        // PARAM_GAIN.description = DESCRIPTION.as_ptr() as *const c_char;
        // PARAM_GAIN.union = FMOD_DSP_PARAMETER_DESC_UNION {
        //     floatdesc: FMOD_DSP_PARAMETER_DESC_FLOAT {
        //         min: 0.0,
        //         max: 0.0,
        //         defaultval: 0.0,
        //         mapping: FMOD_DSP_PARAMETER_FLOAT_MAPPING { type_: FMOD_DSP_PARAMETER_FLOAT_MAPPING_TYPE::FMOD_DSP_PARAMETER_FLOAT_MAPPING_TYPE_LINEAR, piecewiselinearmapping: FMOD_DSP_PARAMETER_FLOAT_MAPPING_PIECEWISE_LINEAR {} },
        //     },
        // };
        //
        // //todo remaining fields of PARAM_GAIN
        // //todo make function to fill in the parameter fields.
        //
        DSP_DESCRIPTION.name = str_to_c_char_array("Phonon Spatializer");
        // DSP_DESCRIPTION.paramdesc = PARAMETERS.as_mut_ptr();

        //static mut desc: FMOD_DSP_DESCRIPTION = DSP_DESCRIPTION.into();
        addr_of_mut!(DSP_DESCRIPTION)
    }
}

fn name32(name: &str) -> [i8; 32] {
    let mut output = [0; 32];
    for (i, ch) in name.as_bytes().iter().enumerate() {
        output[i] = *ch as i8;
    }
    output
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
