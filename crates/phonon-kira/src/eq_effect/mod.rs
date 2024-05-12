use kira::command::{CommandReader, CommandWriter};

pub mod builder;
pub(crate) mod effect;
pub mod handle;

pub(crate) struct CommandWriters {
    set_eq_gains: CommandWriter<[f32; 3]>,
    set_gain: CommandWriter<f32>,
}

pub(crate) struct CommandReaders {
    set_eq_gains: CommandReader<[f32; 3]>,
    set_gain: CommandReader<f32>,
}

pub(crate) fn command_writers_and_readers() -> (CommandWriters, CommandReaders) {
    let (set_eq_gains_writer, set_eq_gains_reader) = ::kira::command::command_writer_and_reader();
    let (set_gain_writer, set_gain_reader) = ::kira::command::command_writer_and_reader();

    let command_writers = CommandWriters {
        set_eq_gains: set_eq_gains_writer,
        set_gain: set_gain_writer,
    };

    let command_readers = CommandReaders {
        set_eq_gains: set_eq_gains_reader,
        set_gain: set_gain_reader,
    };

    (command_writers, command_readers)
}
