use kira::command_writers_and_readers;

pub mod builder;
pub(crate) mod effect;
pub mod handle;

command_writers_and_readers!(
  set_eq_gains: [f32; 3],
    set_gain: f32,
);
