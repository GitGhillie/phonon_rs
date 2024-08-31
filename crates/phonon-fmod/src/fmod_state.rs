use crate::EffectState;
use libfmod::ffi::{FMOD_DEBUG_LEVEL_LOG, FMOD_DSP_STATE, FMOD_OK, FMOD_RESULT};
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_uint};

pub(crate) struct FmodDspState(*mut FMOD_DSP_STATE);

impl FmodDspState {
    pub(crate) fn new(state: *mut FMOD_DSP_STATE) -> Self {
        Self(state)
    }

    pub(crate) unsafe fn get_effect_state(&self) -> *mut EffectState {
        (*self.0).plugindata as *mut EffectState
    }

    pub(crate) unsafe fn get_block_size(&self) -> Result<c_uint, FMOD_RESULT> {
        let functions = (*self.0).functions;
        let block_size_fn = (*functions).getblocksize.unwrap();

        let mut block_size = 0;
        let block_size_ref = &mut block_size;

        let fmod_result = block_size_fn(self.0, block_size_ref);

        if fmod_result == FMOD_OK {
            Ok(block_size)
        } else {
            Err(fmod_result)
        }
    }

    pub(crate) unsafe fn get_sample_rate(&self) -> Result<c_int, FMOD_RESULT> {
        let functions = (*self.0).functions;
        let sample_rate_fn = (*functions).getsamplerate.unwrap();

        let mut sample_rate = 0;
        let sample_rate_ref = &mut sample_rate;

        let fmod_result = sample_rate_fn(self.0, sample_rate_ref);

        if fmod_result == FMOD_OK {
            Ok(sample_rate)
        } else {
            Err(fmod_result)
        }
    }

    pub(crate) unsafe fn log_message(&self, message: &'static str) {
        let functions = (*self.0).functions;
        let log_fn = (*functions).log.unwrap();

        let flags = FMOD_DEBUG_LEVEL_LOG;
        let file = convert_str_to_c_string("testFile");
        let line = 42;
        let function = convert_str_to_c_string("testFunction");
        let message = convert_str_to_c_string(message);

        log_fn(
            flags,
            file.as_ptr(),
            line,
            function.as_ptr(),
            message.as_ptr(),
        );
    }
}

fn convert_str_to_c_string(message: &'static str) -> CString {
    CString::new(message).expect("CString::new failed")
}
