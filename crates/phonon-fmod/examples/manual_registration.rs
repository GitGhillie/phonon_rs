// Mostly copied from libfmod's examples

use std::io;
use std::io::BufRead;
use std::ptr::null_mut;

use libfmod::ffi::{
    FMOD_DSP_PARAMETER_DESC_FLOAT, FMOD_DSP_PARAMETER_DESC_UNION, FMOD_INIT_NORMAL,
    FMOD_LOOP_NORMAL,
};
use libfmod::{DspDescription, DspParameterDesc, DspParameterType, Error, System};
use phonon_fmod::callbacks::{
    create_callback, get_float_callback, process_callback, release_callback, reset_callback,
    set_float_callback, shouldiprocess_callback, sys_deregister_callback, sys_register_callback,
};

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

    // todo move outside of example
    let dspdesc = DspDescription {
        pluginsdkversion: 0,
        name: name32("My first DSP unit"),
        version: 0x00010000,
        numinputbuffers: 1,
        numoutputbuffers: 1,
        create: Some(create_callback),
        release: Some(release_callback),
        reset: Some(reset_callback),
        read: None,
        process: Some(process_callback),
        setposition: None,
        paramdesc: vec![volume_desc],
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

    // todo : move create_dsp and add_dsp to a different example.
    // This example should be minimal.
    let mydsp = system.create_dsp(dspdesc)?;
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
