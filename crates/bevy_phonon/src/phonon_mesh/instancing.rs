use crate::phonon_mesh::material::PhononMaterial;
use crate::phonon_mesh::mesh;
use crate::phonon_plugin::SteamSimulation;
use bevy::asset::{Assets, Handle};
use bevy::ecs::system::SystemParam;
use bevy::log::debug;
use bevy::prelude::{Deref, DerefMut, Mesh, ResMut, Resource, Transform};
use firewheel_phonon::phonon;
use phonon::scene::instanced_mesh::InstancedMesh;
use phonon::scene::static_mesh::StaticMesh;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Resource, Default, Deref, DerefMut)]
pub(crate) struct StaticMeshes(
    HashMap<(Handle<Mesh>, PhononMaterial), Arc<Mutex<phonon::scene::Scene>>>,
);

/// Some information necessary to convert Bevy meshes to Steam Audio meshes
#[derive(SystemParam)]
pub(crate) struct MeshParam<'w> {
    pub bevy_meshes: ResMut<'w, Assets<Mesh>>,
    pub static_meshes: ResMut<'w, StaticMeshes>,
    pub simulator: ResMut<'w, SteamSimulation>,
}

impl<'w> MeshParam<'w> {
    /// Creates a Steam Audio Instanced Mesh from a Bevy Mesh.
    /// If the Bevy mesh has been converted before it will re-use the Steam Audio mesh.
    pub(crate) fn create_instanced_mesh(
        &mut self,
        mesh_handle: &Handle<Mesh>,
        material: &PhononMaterial,
    ) -> Option<Arc<Mutex<InstancedMesh>>> {
        create_instanced_mesh_internal(self, mesh_handle, material)
    }
}

fn create_instanced_mesh_internal(
    mesh_param: &mut MeshParam,
    mesh_handle: &Handle<Mesh>,
    material: &PhononMaterial,
) -> Option<Arc<Mutex<InstancedMesh>>> {
    let static_meshes = &mut mesh_param.static_meshes;
    let meshes = &mesh_param.bevy_meshes;
    let simulator = &mut mesh_param.simulator;
    let scene_root = &mut simulator.scene;

    if let Some(static_mesh_scene) = static_meshes.get(&(mesh_handle.clone(), material.clone())) {
        debug!("Found static mesh, creating instance");
        // Mesh has been converted into phonon mesh before.
        // Turn that mesh into an instanced one, so it can be moved around.
        // todo: Differentiate between set-and-forget and movable audio meshes:
        // Currently to_matrix will be called every frame for every mesh.

        let instanced_mesh = Arc::new(InstancedMesh::new(
            static_mesh_scene.clone(),
            Transform::default().to_matrix(),
        ));
        scene_root.add_instanced_mesh(instanced_mesh.clone());

        Some(instanced_mesh)
    } else {
        debug!("New audio mesh, creating static mesh and instance");
        // Create audio geometry
        if let Some(mesh) = meshes.get(mesh_handle) {
            let audio_mesh: StaticMesh = mesh::try_from(mesh, material.clone()).unwrap();

            // Create sub scene with static mesh, this will later be used to create the instanced mesh
            let mut sub_scene = phonon::scene::Scene::new();

            // Add mesh
            sub_scene.add_static_mesh(Arc::new(audio_mesh));
            sub_scene.commit();

            let sub_scene = Arc::new(Mutex::new(sub_scene));
            static_meshes.insert((mesh_handle.clone(), material.clone()), sub_scene.clone());

            // Turn that mesh into an instanced one, so it can be moved around.
            // todo: Differentiate between set-and-forget and movable audio meshes.
            // Currently to_matrix will be called every frame for every mesh.
            let instanced_mesh = Arc::new(InstancedMesh::new(
                sub_scene,
                Transform::default().to_matrix(),
            ));
            scene_root.add_instanced_mesh(instanced_mesh.clone());

            Some(instanced_mesh)
        } else {
            None // todo: Improve this mess. There is also a bit of duplicated code above
        }
    }
}
