// Mostly copied from libfmod's examples

use std::io;
use std::io::BufRead;

use libfmod::ffi::{
    FMOD_DSP_PARAMETER_DESC_FLOAT, FMOD_DSP_PARAMETER_DESC_UNION, FMOD_INIT_NORMAL,
    FMOD_LOOP_NORMAL,
};
use libfmod::{DspDescription, DspParameterDesc, DspParameterType, Error, System};
use phonon_fmod::create_dsp_description;

fn main() -> Result<(), Error> {
    let system = System::create()?;
    system.init(32, FMOD_INIT_NORMAL, None)?;

    let sound = system.create_sound("./data/audio/windless_slopes.ogg", FMOD_LOOP_NORMAL, None)?;
    system.play_sound(sound, None, false)?;

    let volume_desc = DspParameterDesc {
        type_: DspParameterType::Float,
        name: name16("volume"),
        label: name16("%"),
        description: "linear volume in percent".to_string(),
        union: FMOD_DSP_PARAMETER_DESC_UNION {
            floatdesc: FMOD_DSP_PARAMETER_DESC_FLOAT {
                min: 0.0,
                max: 1.0,
                defaultval: 1.0,
                mapping: Default::default(),
            },
        },
    };

    // todo : move create_dsp and add_dsp to a different example.
    // This example should be minimal.
    let desc = DspDescription::try_from(unsafe { create_dsp_description() })?;
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

fn name16(name: &str) -> [i8; 16] {
    let mut output = [0; 16];
    for (i, ch) in name.as_bytes().iter().enumerate() {
        output[i] = *ch as i8;
    }
    output
}

fn name32(name: &str) -> [i8; 32] {
    let mut output = [0; 32];
    for (i, ch) in name.as_bytes().iter().enumerate() {
        output[i] = *ch as i8;
    }
    output
}
