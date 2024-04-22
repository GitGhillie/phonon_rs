pub mod builder;
pub(crate) mod effect;
pub mod handle;

pub(crate) enum Command {
    SetGains([f32; 3]),
}
