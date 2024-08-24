//! This example shows how to create the Phonon DSP for FMOD
//! through code. This is not recommend (use FMOD Studio instead)
//! but it can be a helpful example for debugging purposes.

// Mostly copied from libfmod's examples

use libfmod::ffi::{
    FMOD_3D_ATTRIBUTES, FMOD_DSP_PARAMETER_3DATTRIBUTES, FMOD_INIT_NORMAL, FMOD_LOOP_NORMAL,
    FMOD_VECTOR,
};
use libfmod::{DspDescription, Error, System};
use phonon_fmod::create_dsp_description;
use std::io;
use std::io::BufRead;
use std::os::raw::c_void;

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
                //mydsp.set_parameter_float(0, 0.2)?;
                let mut attributes = FMOD_DSP_PARAMETER_3DATTRIBUTES {
                    relative: FMOD_3D_ATTRIBUTES {
                        position: FMOD_VECTOR {
                            x: -3.0,
                            y: 0.0,
                            z: 0.0,
                        },
                        velocity: Default::default(),
                        forward: Default::default(),
                        up: Default::default(),
                    },
                    absolute: Default::default(),
                };
                let attributes_ptr = &mut attributes as *mut _ as *mut c_void;
                let attributes_size = std::mem::size_of::<FMOD_DSP_PARAMETER_3DATTRIBUTES>();
                mydsp.set_parameter_data(0, attributes_ptr, attributes_size as u32)?
            }
            4 => {
                //let (value, _) = mydsp.get_parameter_float(0, 0)?;
                //println!("volume: {}", value);
            }
            _ => {}
        }
    }
    let info = mydsp.get_parameter_info(0)?;
    //println!("default: {}", unsafe { info.union.floatdesc.defaultval });

    println!("Press Enter to exit.");
    let stdin = io::stdin();
    stdin.lock().lines().next().unwrap().unwrap();

    system.release()
}
