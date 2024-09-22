//! This example shows how to create the Phonon DSP for FMOD
//! through code. This is not recommend (use FMOD Studio instead)
//! but it can be a helpful example for debugging purposes.

// Mostly copied from libfmod's examples

use libfmod::ffi::{
    FMOD_3D_ATTRIBUTES, FMOD_DSP_PARAMETER_3DATTRIBUTES, FMOD_INIT_NORMAL, FMOD_LOOP_NORMAL,
    FMOD_VECTOR,
};
use libfmod::{Error, System};
use phonon::simulators::direct_simulator::DirectSoundPath;
use phonon_fmod::create_dsp_description;
use std::io;
use std::io::BufRead;
use std::os::raw::c_void;

fn main() -> Result<(), Error> {
    let system = System::create()?;
    system.init(32, FMOD_INIT_NORMAL, None)?;

    let sound = system.create_sound("./data/audio/windless_slopes.ogg", FMOD_LOOP_NORMAL, None)?;
    system.play_sound(sound, None, false)?;

    let desc = create_dsp_description();
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
                            x: -20.0,
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

                let mut direct_sound_path = DirectSoundPath {
                    distance_attenuation: 0.5,
                    air_absorption: [1.0, 1.0, 1.0],
                    delay: 0.0,
                    occlusion: 1.0,
                    transmission: [1.0, 1.0, 1.0],
                    directivity: 0.0,
                };
                let sound_path_ptr = &mut direct_sound_path as *mut _ as *mut c_void;
                let sound_path_size = std::mem::size_of::<DirectSoundPath>();

                // set 3d attributes
                mydsp.set_parameter_data(0, attributes_ptr, attributes_size as u32)?;
                // enable/disable distance attenuation
                mydsp.set_parameter_int(2, 1)?;
                // set direct sound path
                mydsp.set_parameter_data(10, sound_path_ptr, sound_path_size as u32)?
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
