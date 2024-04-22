pub mod builder;
pub(crate) mod effect;
pub mod handle;

pub(crate) enum Command {
    SetEqGains([f32; 3]),
}
