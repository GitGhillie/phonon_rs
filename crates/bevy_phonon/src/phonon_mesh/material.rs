// Mostly copied from https://github.com/Aceeri/steam-audio-rs/blob/master/steam-audio/src/simulation/material.rs

use bevy::prelude::{Deref, DerefMut};
use firewheel_phonon::phonon;
use phonon::scene::material::Material;
use std::hash::{DefaultHasher, Hash, Hasher};

//todo: It shouldn't be necessary to make a newtype
/// Acoustic properties of a surface.
#[derive(Debug, Clone, PartialEq, Deref, DerefMut)]
pub struct PhononMaterial(Material);

impl Eq for PhononMaterial {}
impl Hash for PhononMaterial {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut hasher = DefaultHasher::new();

        for num in self.absorption {
            num.to_bits().hash(&mut hasher);
        }

        self.scattering.to_bits().hash(&mut hasher);

        for num in self.transmission {
            num.to_bits().hash(&mut hasher);
        }

        hasher.finish().hash(state);
    }
}

impl From<&PhononMaterial> for Material {
    fn from(material: &PhononMaterial) -> Material {
        Material {
            absorption: material.absorption,
            scattering: material.scattering,
            transmission: material.transmission,
        }
    }
}

impl From<PhononMaterial> for Material {
    fn from(material: PhononMaterial) -> Material {
        Material {
            absorption: material.absorption,
            scattering: material.scattering,
            transmission: material.transmission,
        }
    }
}

impl Default for PhononMaterial {
    fn default() -> Self {
        materials::GENERIC
    }
}

#[allow(unused)]
pub mod materials {
    use super::PhononMaterial;
    use firewheel_phonon::phonon;
    use phonon::scene::material::Material;

    pub const GENERIC: PhononMaterial = PhononMaterial(Material {
        absorption: [0.10, 0.20, 0.30],
        scattering: 0.05,
        transmission: [0.100, 0.050, 0.030],
    });
    pub const BRICK: PhononMaterial = PhononMaterial(Material {
        absorption: [0.03, 0.04, 0.07],
        scattering: 0.05,
        transmission: [0.015, 0.015, 0.015],
    });
    pub const CONCRETE: PhononMaterial = PhononMaterial(Material {
        absorption: [0.05, 0.07, 0.08],
        scattering: 0.05,
        transmission: [0.015, 0.002, 0.001],
    });
    pub const CERAMIC: PhononMaterial = PhononMaterial(Material {
        absorption: [0.01, 0.02, 0.02],
        scattering: 0.05,
        transmission: [0.060, 0.044, 0.011],
    });
    pub const GRAVEL: PhononMaterial = PhononMaterial(Material {
        absorption: [0.60, 0.70, 0.80],
        scattering: 0.05,
        transmission: [0.031, 0.012, 0.008],
    });
    pub const CARPET: PhononMaterial = PhononMaterial(Material {
        absorption: [0.24, 0.69, 0.73],
        scattering: 0.90,
        transmission: [0.020, 0.005, 0.003],
    });
    pub const GLASS: PhononMaterial = PhononMaterial(Material {
        absorption: [0.06, 0.03, 0.02],
        scattering: 0.05,
        transmission: [0.060, 0.044, 0.011],
    });
    pub const PLASTER: PhononMaterial = PhononMaterial(Material {
        absorption: [0.12, 0.06, 0.04],
        scattering: 0.05,
        transmission: [0.056, 0.056, 0.004],
    });
    pub const WOOD: PhononMaterial = PhononMaterial(Material {
        absorption: [0.11, 0.07, 0.06],
        scattering: 0.05,
        transmission: [0.070, 0.014, 0.005],
    });
    pub const METAL: PhononMaterial = PhononMaterial(Material {
        absorption: [0.20, 0.07, 0.06],
        scattering: 0.05,
        transmission: [0.200, 0.025, 0.010],
    });
    pub const ROCK: PhononMaterial = PhononMaterial(Material {
        absorption: [0.13, 0.20, 0.24],
        scattering: 0.05,
        transmission: [0.015, 0.002, 0.001],
    });
}
