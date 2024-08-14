//! FMOD Plugin for the phonon crate.

pub mod callbacks; // todo should probably not be pub

use crate::callbacks::{
    create_callback, get_float_callback, process_callback, release_callback, reset_callback,
    set_float_callback, shouldiprocess_callback, sys_deregister_callback, sys_register_callback,
};
use libfmod::ffi::{
    FMOD_DSP_DESCRIPTION, FMOD_DSP_PARAMETER_DESC, FMOD_DSP_PARAMETER_DESC_FLOAT,
    FMOD_DSP_PARAMETER_DESC_UNION, FMOD_DSP_PARAMETER_FLOAT_MAPPING, FMOD_DSP_PARAMETER_TYPE_FLOAT,
    FMOD_PLUGIN_SDK_VERSION,
};
use std::ffi::CString;
use std::mem::MaybeUninit;
use std::os::raw::c_char;
use std::ptr::addr_of_mut;
use std::ptr::null_mut;

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

fn linear_to_db(lin_value: f32) -> f32 {
    if lin_value <= 0.0 {
        FMOD_GAIN_PARAM_GAIN_MIN
    } else {
        20.0 * lin_value.log10()
    }
}

pub struct FmodGainState {
    target_gain: f32,
    current_gain: f32,
    ramp_samples_left: i32,
}

impl FmodGainState {
    fn reset(&mut self) {
        self.current_gain = self.target_gain;
        self.ramp_samples_left = 0;
    }

    fn set_gain(&mut self, gain: f32) {
        self.target_gain = db_to_linear(gain);
        self.ramp_samples_left = FMOD_GAIN_RAMP_COUNT;
    }

    fn get_gain(&self) -> f32 {
        linear_to_db(self.target_gain)
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
