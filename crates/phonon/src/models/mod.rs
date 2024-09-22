//! Models for audio propagation/attenuation. Each should have a good default
//! but the user should be able to use a custom model as well.

pub mod air_absorption;
pub mod directivity;
pub mod distance_attenuation;
pub mod propagation_medium;
