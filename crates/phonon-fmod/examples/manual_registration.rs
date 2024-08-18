//! This example shows how to register the custom DSP.
//! If the Phonon FMOD plugin is loaded in FMOD Studio
//! (by placing the DLL in the project's Plugins folder)
//! then this step is necessary on the application side.
//!
//! If the DSP is created manually (not recommended)
//! then the create_dsp example should be sufficient.

use std::io;
use std::io::BufRead;

use libfmod::ffi::FMOD_INIT_NORMAL;
use libfmod::{DspDescription, Error, System};
use phonon_fmod::create_dsp_description;

fn main() -> Result<(), Error> {
    let system = System::create()?;
    system.init(32, FMOD_INIT_NORMAL, None)?;

    let desc = DspDescription::try_from(create_dsp_description())?;
    system.register_dsp(desc)?;

    println!("Press Enter to exit.");
    let stdin = io::stdin();
    stdin.lock().lines().next().unwrap().unwrap();

    system.release()
}
