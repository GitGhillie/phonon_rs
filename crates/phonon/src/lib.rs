//! A community effort to rewrite Valve's Steam Audio into a Rust library.

// todo: This one is not worth enabling at the moment
#![allow(clippy::needless_range_loop)]

#[cfg(feature = "serde-serialize")]
#[macro_use]
extern crate serde;

pub mod context;
pub mod dsp;
pub mod effects;
pub mod models;
pub mod scene;
pub mod simulators;
