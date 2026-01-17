use bevy::diagnostic::{DiagnosticPath, Diagnostics};
use bevy::prelude::*;

use crate::phonon_plugin::SteamSimulation;

pub(crate) const MESH_COUNT_STATIC: DiagnosticPath =
    DiagnosticPath::const_new("phonon_mesh_count_static");
pub(crate) const MESH_COUNT_INSTANCED: DiagnosticPath =
    DiagnosticPath::const_new("phonon_mesh_count_instanced");

// Note: This is a polling setup and might miss things
pub(crate) fn count_meshes(mut diagnostics: Diagnostics, simulator: Res<SteamSimulation>) {
    let scene = &simulator.scene;
    diagnostics.add_measurement(&MESH_COUNT_STATIC, || scene.get_num_meshes_static() as f64);
    diagnostics.add_measurement(&MESH_COUNT_INSTANCED, || {
        scene.get_num_meshes_instanced() as f64
    });
}
