pub(crate) mod instancing;
pub mod material;
mod mesh;

pub use material::materials;

use crate::phonon_mesh::instancing::MeshParam;
use crate::phonon_mesh::material::PhononMaterial;
use bevy::prelude::*;
use firewheel_phonon::phonon;
use phonon::scene::instanced_mesh::InstancedMesh;
use std::sync::{Arc, Mutex};

#[derive(Component, Default)]
pub struct NeedsAudioMesh(pub PhononMaterial);

#[derive(Component)]
pub(crate) struct PhononMesh(Arc<Mutex<InstancedMesh>>);

/// If an entity with a `NeedsAudioMesh` marker and a Bevy mesh exist, it will attempt to convert
/// the mesh to a Steam Audio mesh and add it to the audio world.
pub(crate) fn register_audio_meshes(
    mut commands: Commands,
    mut mesh_param: MeshParam,
    mut object_query: Query<(Entity, &Mesh3d, &NeedsAudioMesh)>,
) {
    for (ent, mesh_handle, requested_material) in &mut object_query {
        let instanced_mesh = mesh_param
            .create_instanced_mesh(mesh_handle, &requested_material.0)
            .unwrap();

        let scene_root = &mut mesh_param.simulator.scene;
        scene_root.commit();

        commands.entity(ent).insert(PhononMesh(instanced_mesh));
        commands.entity(ent).remove::<NeedsAudioMesh>();
    }
}

//Changed<GlobalTransform> or Changed Mesh? not worth it probably
pub(crate) fn update_audio_mesh_transforms(
    mut object_query: Query<(&GlobalTransform, &mut PhononMesh)>,
) {
    for (transform, mut audio_instance) in &mut object_query {
        let instanced_mesh = &mut audio_instance.0;
        instanced_mesh
            .lock()
            .unwrap()
            .set_transform(transform.to_matrix())
    }
}
