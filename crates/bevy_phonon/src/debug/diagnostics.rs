use bevy::diagnostic::{DiagnosticPath, Diagnostics};
use bevy::prelude::*;

use crate::phonon_mesh::instancing::StaticMeshes;
use crate::phonon_plugin::SteamSimulation;

pub(crate) const MESH_COUNT_STATIC: DiagnosticPath =
    DiagnosticPath::const_new("phonon_mesh_count_static");
pub(crate) const MESH_COUNT_INSTANCED: DiagnosticPath =
    DiagnosticPath::const_new("phonon_mesh_count_instanced");

// Note: This is a polling setup and might miss things
pub(crate) fn count_meshes(
    mut diagnostics: Diagnostics,
    simulator: Res<SteamSimulation>,
    meshes_resource: Res<StaticMeshes>,
) {
    // This is technically the number of sub-scenes of the root scene.
    // However, in the current implementation the sub-scenes do not have any
    // sub-scenes themselves, and each sub-scene only has one static mesh.
    let num_static_meshes = meshes_resource.len();

    diagnostics.add_measurement(&MESH_COUNT_STATIC, || num_static_meshes as f64);
    diagnostics.add_measurement(&MESH_COUNT_INSTANCED, || {
        simulator.scene.get_num_meshes_instanced() as f64
    });
}
