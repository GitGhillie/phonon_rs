//! Everything related to ray tracing and representing a scene in 3D space.

pub mod coordinate_space;
pub mod hit;
pub mod instanced_mesh;
pub mod material;
pub mod mesh;
pub mod ray;
pub mod sampling;
pub mod scene;
pub mod sphere;
pub mod static_mesh;
pub mod triangle;

pub use scene::Scene;
