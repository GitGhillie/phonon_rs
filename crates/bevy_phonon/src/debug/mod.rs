mod diagnostics;
mod gizmos;

use bevy::{
    diagnostic::{Diagnostic, RegisterDiagnostic},
    prelude::*,
};

use crate::debug::{
    diagnostics::{MESH_COUNT_INSTANCED, MESH_COUNT_STATIC, count_meshes},
    gizmos::{AudioGizmoConfigGroup, visualize_sources},
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.register_diagnostic(Diagnostic::new(MESH_COUNT_STATIC).with_suffix(" meshes"))
            .register_diagnostic(Diagnostic::new(MESH_COUNT_INSTANCED).with_suffix(" meshes"))
            .add_systems(PostUpdate, count_meshes)
            .init_gizmo_group::<AudioGizmoConfigGroup>()
            .add_systems(
                PostUpdate,
                visualize_sources.after(TransformSystems::Propagate),
            );
    }
}
