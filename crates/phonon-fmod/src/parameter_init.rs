//
// Copyright 2017-2023 Valve Corporation.
// Copyright 2024 phonon_rs contributors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use crate::str_to_c_char_array;
use libfmod::ffi::{
    FMOD_DSP_PARAMETER_DATA_TYPE, FMOD_DSP_PARAMETER_DATA_TYPE_3DATTRIBUTES,
    FMOD_DSP_PARAMETER_DATA_TYPE_USER, FMOD_DSP_PARAMETER_DESC_BOOL, FMOD_DSP_PARAMETER_DESC_DATA,
    FMOD_DSP_PARAMETER_DESC_FLOAT, FMOD_DSP_PARAMETER_DESC_INT, FMOD_DSP_PARAMETER_DESC_UNION,
};
use libfmod::{DspParameterDesc, DspParameterType};
use std::ffi::CString;
use std::os::raw::{c_char, c_int};

fn create_param_data(
    name: &str,
    description: &'static str,
    datatype: FMOD_DSP_PARAMETER_DATA_TYPE,
) -> DspParameterDesc {
    DspParameterDesc {
        type_: DspParameterType::Data,
        name: str_to_c_char_array(name),
        label: str_to_c_char_array(""),
        description: description.to_string(),
        union: FMOD_DSP_PARAMETER_DESC_UNION {
            datadesc: FMOD_DSP_PARAMETER_DESC_DATA { datatype },
        },
    }
}

fn create_param_float(
    name: &str,
    description: &'static str,
    min: f32,
    max: f32,
    default: f32,
) -> DspParameterDesc {
    DspParameterDesc {
        type_: DspParameterType::Float,
        name: str_to_c_char_array(name),
        label: str_to_c_char_array("%"),
        description: description.to_string(),
        union: FMOD_DSP_PARAMETER_DESC_UNION {
            floatdesc: FMOD_DSP_PARAMETER_DESC_FLOAT {
                min,
                max,
                defaultval: default,
                mapping: Default::default(),
            },
        },
    }
}

fn create_param_bool(
    name: &str,
    description: &'static str,
    default: bool,
    value_names: [&'static str; 2],
) -> DspParameterDesc {
    let value_names_c: Vec<*mut c_char> = value_names
        .into_iter()
        .map(|value_name| {
            CString::new(value_name)
                .unwrap_or(CString::from(c"err!"))
                .into_raw()
        })
        .collect();

    DspParameterDesc {
        type_: DspParameterType::Bool,
        name: str_to_c_char_array(name),
        label: str_to_c_char_array(""),
        description: description.to_string(),
        union: FMOD_DSP_PARAMETER_DESC_UNION {
            booldesc: FMOD_DSP_PARAMETER_DESC_BOOL {
                defaultval: default as c_int,
                valuenames: Box::into_raw(value_names_c.into_boxed_slice()) as *const *const c_char,
            },
        },
    }
}

fn create_param_int(
    name: &str,
    description: &'static str,
    value_names: Vec<&'static str>,
) -> DspParameterDesc {
    let value_names_c: Vec<*mut c_char> = value_names
        .into_iter()
        .map(|value_name| {
            CString::new(value_name)
                .unwrap_or(CString::from(c"err!"))
                .into_raw()
        })
        .collect();

    DspParameterDesc {
        type_: DspParameterType::Int,
        name: str_to_c_char_array(name),
        label: str_to_c_char_array(""),
        description: description.to_string(),
        union: FMOD_DSP_PARAMETER_DESC_UNION {
            intdesc: FMOD_DSP_PARAMETER_DESC_INT {
                min: 0,
                max: (value_names_c.len() - 1) as c_int,
                defaultval: 0,
                goestoinf: 0,
                valuenames: Box::into_raw(value_names_c.into_boxed_slice()) as *const *const c_char,
            },
        },
    }
}

fn create_param_apply(name: &str, description: &'static str) -> DspParameterDesc {
    let param_str_off = "Off";
    let param_str_physics = "Physics-Based";
    let param_str_curve = "Curve-Driven";

    create_param_int(
        name,
        description,
        vec![param_str_off, param_str_physics, param_str_curve],
    )
}

pub(crate) fn init_parameters() -> Vec<DspParameterDesc> {
    let param_source = create_param_data(
        "SourcePos",
        "Position of the source.",
        FMOD_DSP_PARAMETER_DATA_TYPE_3DATTRIBUTES,
    );

    let param_overall_gain = create_param_data(
        "OverallGain",
        "Overall gain.",
        FMOD_DSP_PARAMETER_DATA_TYPE_3DATTRIBUTES,
    );

    let param_apply_distance_attenuation =
        create_param_apply("ApplyDA", "Apply distance attenuation.");
    let param_apply_air_absorption = create_param_apply("ApplyAA", "Apply air absorption.");
    let param_apply_directivity = create_param_apply("ApplyDir", "Apply directivity.");
    let param_apply_occlusion = create_param_apply("ApplyOc", "Apply occlusion.");
    let param_apply_transmission = create_param_apply("ApplyTrans", "Apply transmission.");
    let param_apply_reflections = create_param_bool(
        "ApplyReflections",
        "Apply reflections.",
        true,
        ["Off", "On"],
    );
    let param_apply_pathing =
        create_param_bool("ApplyPathing", "Apply pathing.", true, ["Off", "On"]);

    let param_hrtf_interpolation = create_param_int(
        "HrtfInterp",
        "HRTF Interpolation.",
        vec!["Nearest", "Bilinear"],
    );

    let param_direct_sound_path = create_param_data(
        "DirectSoundPath",
        "DirectSoundPath simulation output.",
        FMOD_DSP_PARAMETER_DATA_TYPE_USER,
    );

    let param_directivity_dipole_weight =
        create_param_float("DipoleWeight", "Directivity dipole weight.", 0.0, 1.0, 1.0);
    let param_directivity_dipole_power =
        create_param_float("DipolePower", "Directivity dipole power.", 0.0, 4.0, 1.0);

    let param_direct_binaural = create_param_bool(
        "DirectBinaural",
        "Apply HRTF to direct path.",
        true,
        ["Off", "On"],
    );

    vec![
        param_source,
        param_overall_gain,
        param_apply_distance_attenuation,
        param_apply_air_absorption,
        param_apply_directivity,
        param_apply_occlusion,
        param_apply_transmission,
        param_apply_reflections,
        param_apply_pathing,
        param_hrtf_interpolation,
        param_direct_sound_path,
        param_directivity_dipole_weight,
        param_directivity_dipole_power,
        param_direct_binaural,
    ]
}
