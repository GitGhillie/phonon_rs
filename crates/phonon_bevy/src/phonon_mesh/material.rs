// Mostly copied from https://github.com/Aceeri/steam-audio-rs/blob/master/steam-audio/src/simulation/material.rs

use std::hash::{DefaultHasher, Hash, Hasher};

/// Acoustic properties of a surface.
#[derive(Debug, Clone, PartialEq)]
pub struct PhononMaterial {
    /// Specified in 3 frequency bands of 400 Hz, 2.5KHz, and 15 KHz.
    pub absorption: [f32; 3],
    pub scattering: f32,
    pub transmission: [f32; 3],
}

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

impl Into<steamaudio::scene::Material> for &PhononMaterial {
    fn into(self) -> steamaudio::scene::Material {
        steamaudio::scene::Material {
            absorption: self.absorption,
            scattering: self.scattering,
            transmission: self.transmission,
        }
    }
}

impl Into<steamaudio::scene::Material> for PhononMaterial {
    fn into(self) -> steamaudio::scene::Material {
        steamaudio::scene::Material {
            absorption: self.absorption,
            scattering: self.scattering,
            transmission: self.transmission,
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

    pub const GENERIC: PhononMaterial = PhononMaterial {
        absorption: [0.10, 0.20, 0.30],
        scattering: 0.05,
        transmission: [0.100, 0.050, 0.030],
    };
    pub const BRICK: PhononMaterial = PhononMaterial {
        absorption: [0.03, 0.04, 0.07],
        scattering: 0.05,
        transmission: [0.015, 0.015, 0.015],
    };
    pub const CONCRETE: PhononMaterial = PhononMaterial {
        absorption: [0.05, 0.07, 0.08],
        scattering: 0.05,
        transmission: [0.015, 0.002, 0.001],
    };
    pub const CERAMIC: PhononMaterial = PhononMaterial {
        absorption: [0.01, 0.02, 0.02],
        scattering: 0.05,
        transmission: [0.060, 0.044, 0.011],
    };
    pub const GRAVEL: PhononMaterial = PhononMaterial {
        absorption: [0.60, 0.70, 0.80],
        scattering: 0.05,
        transmission: [0.031, 0.012, 0.008],
    };
    pub const CARPET: PhononMaterial = PhononMaterial {
        absorption: [0.24, 0.69, 0.73],
        scattering: 0.90,
        transmission: [0.020, 0.005, 0.003],
    };
    pub const GLASS: PhononMaterial = PhononMaterial {
        absorption: [0.06, 0.03, 0.02],
        scattering: 0.05,
        transmission: [0.060, 0.044, 0.011],
    };
    pub const PLASTER: PhononMaterial = PhononMaterial {
        absorption: [0.12, 0.06, 0.04],
        scattering: 0.05,
        transmission: [0.056, 0.056, 0.004],
    };
    pub const WOOD: PhononMaterial = PhononMaterial {
        absorption: [0.11, 0.07, 0.06],
        scattering: 0.05,
        transmission: [0.070, 0.014, 0.005],
    };
    pub const METAL: PhononMaterial = PhononMaterial {
        absorption: [0.20, 0.07, 0.06],
        scattering: 0.05,
        transmission: [0.200, 0.025, 0.010],
    };
    pub const ROCK: PhononMaterial = PhononMaterial {
        absorption: [0.13, 0.20, 0.24],
        scattering: 0.05,
        transmission: [0.015, 0.002, 0.001],
    };
}
