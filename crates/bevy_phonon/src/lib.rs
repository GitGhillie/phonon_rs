pub mod phonon_mesh;
pub mod phonon_plugin;

pub mod prelude {
    pub use crate::phonon_mesh::material::materials;
    pub use crate::phonon_mesh::material::PhononMaterial;
    pub use crate::phonon_mesh::NeedsAudioMesh;
    pub use crate::phonon_plugin::PhononPlugin;
}

use bevy::ecs::component::Component;
pub use firewheel_phonon::effects;

#[derive(Component)]
pub struct AudioListener;
