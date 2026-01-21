use bevy::mesh::{Indices, PrimitiveTopology, VertexAttributeValues};
use bevy::prelude::Mesh;

use crate::phonon_mesh::material::PhononMaterial;
use firewheel_phonon::phonon;
use phonon::scene::material::Material;
use phonon::scene::static_mesh::StaticMesh;

#[derive(Debug, Clone)]
pub enum AudioMeshError {
    NoVertices,
    #[expect(dead_code, reason = "Error is currently unwrapped")]
    NonTrianglePrimitiveTopology(PrimitiveTopology),
}

// Original code from https://github.com/Aceeri/bevy-steam-audio/blob/main/src/source.rs
pub fn try_from(mesh: &Mesh, material: PhononMaterial) -> Result<StaticMesh, AudioMeshError> {
    let triangles = match mesh.indices() {
        Some(indices) => {
            let indices: Vec<_> = match indices {
                Indices::U16(indices) => indices.iter().map(|indices| *indices as u32).collect(),
                Indices::U32(indices) => indices.clone(),
            };

            match mesh.primitive_topology() {
                PrimitiveTopology::TriangleList => indices
                    .chunks_exact(3)
                    .map(|chunk| [chunk[0], chunk[1], chunk[2]])
                    .collect(),
                PrimitiveTopology::TriangleStrip => {
                    let mut indices: Vec<_> = indices
                        .windows(3)
                        .map(|indices| [indices[0], indices[1], indices[2]])
                        .collect();

                    for (index, indices) in indices.iter_mut().enumerate() {
                        if (index + 1) % 2 == 0 {
                            *indices = [indices[1], indices[0], indices[2]];
                        }
                    }

                    indices
                }
                topology => return Err(AudioMeshError::NonTrianglePrimitiveTopology(topology)),
            }
        }
        None => Vec::new(),
    };

    let vertices = match mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
        Some(VertexAttributeValues::Float32x3(vertices)) => {
            vertices.iter().map(|a| (*a).into()).collect()
        }
        _ => return Err(AudioMeshError::NoVertices),
    };

    let material: Material = material.into();
    let materials = vec![material];
    let material_indices = triangles.iter().map(|_| 0 /* GENERIC index */).collect();

    Ok(StaticMesh::new_static_mesh(
        vertices,
        triangles,
        material_indices,
        materials,
    ))
}
