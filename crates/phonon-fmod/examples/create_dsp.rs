//! This example shows how to create the Phonon DSP for FMOD
//! through code. This is not recommend (use FMOD Studio instead)
//! but it can be a helpful example for debugging purposes.

// Mostly copied from libfmod's examples

use std::io;
use std::io::BufRead;

use libfmod::ffi::{FMOD_INIT_NORMAL, FMOD_LOOP_NORMAL};
use libfmod::{DspDescription, Error, System};
use phonon_fmod::create_dsp_description;

fn main() -> Result<(), Error> {
    let system = System::create()?;
    system.init(32, FMOD_INIT_NORMAL, None)?;

    let sound = system.create_sound("./data/audio/windless_slopes.ogg", FMOD_LOOP_NORMAL, None)?;
    system.play_sound(sound, None, false)?;

    let desc = DspDescription::try_from(create_dsp_description())?;
    let mydsp = system.create_dsp(desc)?;
    let mastergroup = system.get_master_channel_group()?;
    mastergroup.add_dsp(0, mydsp)?;

    for step in 0..5 {
        match step {
            1 => {
                mydsp.set_bypass(true)?;
            }
            2 => {
                mydsp.set_bypass(false)?;
            }
            3 => {
                mydsp.set_parameter_float(0, 0.2)?;
            }
            4 => {
                let (value, _) = mydsp.get_parameter_float(0, 0)?;
                println!("volume: {}", value);
            }
            _ => {}
        }
    }
    let info = mydsp.get_parameter_info(0)?;
    println!("default: {}", unsafe { info.union.floatdesc.defaultval });

    println!("Press Enter to exit.");
    let stdin = io::stdin();
    stdin.lock().lines().next().unwrap().unwrap();

    system.release()
}
