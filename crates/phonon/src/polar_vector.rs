use glam::Vec3;
use std::f32::consts::{FRAC_PI_2, PI};

const EPSILON: f32 = 0.00001; // 1e-5

/// Represents a point in 3D space using spherical polar coordinates. Elevation
/// is measured in the range [-Pi/2, Pi/2] from the horizontal, and azimuth is
/// measured in the range [0, 2Pi] from straight ahead, going counter-clockwise.
pub struct SphericalVec3 {
    /// The radius, i.e., the distance of the point from the origin.
    pub radius: f32,
    /// The elevation angle.
    pub elivation: f32,
    /// The azimuth angle.
    pub azimuth: f32,
}

impl From<Vec3> for SphericalVec3 {
    fn from(cartesian: Vec3) -> SphericalVec3 {
        let radius = cartesian.length();
        let elivation = f32::asin(cartesian.y / radius);
        let mut azimuth;
        if (f32::abs(elivation - FRAC_PI_2) < EPSILON)
            || (f32::abs(elivation + FRAC_PI_2) < EPSILON)
        {
            azimuth = 0.0;
        } else {
            azimuth = (PI + f32::atan2(cartesian.x, cartesian.z)) % (2.0 * PI)
        }
        SphericalVec3 {
            radius,
            elivation,
            azimuth,
        }
    }
}

impl From<SphericalVec3> for Vec3 {
    fn from(spherical: SphericalVec3) -> Vec3 {
        let SphericalVec3 {
            radius,
            elivation,
            azimuth,
        } = spherical;
        Vec3 {
            x: radius * elivation.cos() * -azimuth.sin(),
            y: radius * elivation.sin(),
            z: radius * elivation.cos() * -azimuth.cos(),
        }
    }
}

impl Default for SphericalVec3 {
    fn default() -> SphericalVec3 {
        SphericalVec3 {
            radius: 0.0,
            elivation: 0.0,
            azimuth: 0.0,
        }
    }
}

impl SphericalVec3 {
    // Creates a new polar vector from radius, elivation and azimuth.
    pub fn new(radius: f32, elivation: f32, azimuth: f32) -> SphericalVec3 {
        SphericalVec3 {
            radius,
            elivation,
            azimuth,
        }
    }
}

// Represents a point in 3D space using interaural polar coordinates. Azimuth is measured in the range
// [-Pi/2, Pi/2] from straight ahead, and elevation is measured in the range [0, 2Pi] from downwards.
pub struct InterauralVec3 {
    /// The radius, i.e., the distance of the point from the origin.
    pub radius: f32,
    /// The elevation angle.
    pub elivation: f32,
    /// The azimuth angle.
    pub azimuth: f32,
}

impl From<Vec3> for InterauralVec3 {
    fn from(cartesian: Vec3) -> InterauralVec3 {
        let radius = cartesian.length();
        let azimuth = f32::asin(cartesian.x / radius);
        let mut elivation;
        if (f32::abs(azimuth - FRAC_PI_2) < EPSILON) || (f32::abs(azimuth + FRAC_PI_2) < EPSILON) {
            elivation = 0.0;
        } else {
            elivation = (PI + f32::atan2(cartesian.z, cartesian.y)) % (2.0 * PI)
        }
        InterauralVec3 {
            radius,
            elivation,
            azimuth,
        }
    }
}

impl From<InterauralVec3> for Vec3 {
    fn from(interaural: InterauralVec3) -> Vec3 {
        let InterauralVec3 {
            radius,
            elivation,
            azimuth,
        } = interaural;
        Vec3 {
            x: radius * azimuth.sin(),
            y: radius * azimuth.cos() * -elivation.cos(),
            z: radius * azimuth.cos() * -elivation.sin(),
        }
    }
}

impl From<SphericalVec3> for InterauralVec3 {
    fn from(spherical: SphericalVec3) -> InterauralVec3 {
        Vec3::from(spherical).into()
    }
}

impl From<InterauralVec3> for SphericalVec3 {
    fn from(interaural: InterauralVec3) -> SphericalVec3 {
        Vec3::from(interaural).into()
    }
}

impl Default for InterauralVec3 {
    fn default() -> InterauralVec3 {
        InterauralVec3 {
            radius: 0.0,
            elivation: 0.0,
            azimuth: 0.0,
        }
    }
}

impl InterauralVec3 {
    // Creates a new polar vector from radius, elivation and azimuth.
    pub fn new(radius: f32, elivation: f32, azimuth: f32) -> InterauralVec3 {
        InterauralVec3 {
            radius,
            elivation,
            azimuth,
        }
    }
}
