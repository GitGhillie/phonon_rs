// todo: This is probably cursed. Needed in the `AudioBuffer` `write` fn.
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

pub mod audio_buffer;
pub mod bands;
pub mod context;
pub mod coordinate_space;
pub mod delay;
pub mod iir;
pub mod reverb_effect;
pub mod reverb_estimator;
pub mod eq_effect;
