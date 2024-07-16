//! FMOD Plugin for the phonon crate.
//!
//! On the FMOD Studio side the plugin needs to be built as a dynamic library
//! and loaded by placing it in one of the folders indicated here:
//! https://www.fmod.com/docs/2.02/studio/plugin-reference.html#loading-plug-ins
//!
//! On the application side the plugin can either be dynamically or statically linked.
//! By default, this should be done statically.

// mod ffi {
//     #![allow(non_snake_case)]
//     #![allow(non_camel_case_types)]
//     #![allow(non_upper_case_globals)]
//
//     include!(concat!(env!("OUT_DIR"), "/bindgen.rs"));
// }

use std::ffi::{c_char, CString};

// todo remove file and use the generated bindings
mod ffi;

static mut DSP_DESCRIPTION: ffi::FMOD_DSP_DESCRIPTION = ffi::FMOD_DSP_DESCRIPTION {
    pluginsdkversion: ffi::FMOD_PLUGIN_SDK_VERSION,
    name: [0; 32],
    version: 1,
    numinputbuffers: 1,
    numoutputbuffers: 1,
    create: None,
    release: None,
    reset: None,
    read: None,
    process: None,
    setposition: None,
    numparameters: 0,
    paramdesc: std::ptr::null_mut(),
    setparameterfloat: None,
    setparameterint: None,
    setparameterbool: None,
    setparameterdata: None,
    getparameterfloat: None,
    getparameterint: None,
    getparameterbool: None,
    getparameterdata: None,
    shouldiprocess: None,
    userdata: std::ptr::null_mut(),
    sys_register: None,
    sys_deregister: None,
    sys_mix: None,
};

/// FMOD will call this function load the plugin defined by FMOD_DSP_DESCRIPTION.
/// See https://fmod.com/docs/2.02/api/white-papers-dsp-plugin-api.html#building-a-plug-in
#[no_mangle]
extern "C" fn FMODGetDSPDescription() -> *mut ffi::FMOD_DSP_DESCRIPTION {
    unsafe {
        DSP_DESCRIPTION.name = str_to_c_char_array("Phonon Spatializer");

        &mut DSP_DESCRIPTION
    }
}

fn str_to_c_char_array(input: &str) -> [c_char; 32] {
    let mut array: [c_char; 32] = [0; 32];

    // Convert the input &str to a CString, adding a null terminator
    let c_string = CString::new(input).expect("CString::new failed");

    // Get the byte slice of the CString
    let bytes = c_string.as_bytes();

    // Ensure the byte slice fits within the array
    if bytes.len() > 32 {
        panic!("String is too long to fit in [c_char; 32]");
    }

    // Copy the bytes into the array
    for (i, &byte) in bytes.iter().enumerate() {
        array[i] = byte as c_char;
    }

    array
}
