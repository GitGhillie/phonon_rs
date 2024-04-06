// todo: This is probably cursed. Needed in the `AudioBuffer` `write` fn.
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub mod audio_buffer;
pub mod bands;
pub mod coordinate_space;
pub mod delay;
pub mod iir;
pub mod reverb_effect;
pub mod reverb_estimator;
pub mod context;
pub mod simulation_data;
pub mod direct_simulator;
pub mod sampling;
