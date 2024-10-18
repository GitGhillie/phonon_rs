use kira::command::{CommandReader, CommandWriter};
use phonon::effects::binaural::BinauralEffectParameters;
use phonon::effects::direct::DirectEffectParameters;

pub mod builder;
pub(crate) mod effect;
pub mod handle;

pub(crate) struct CommandWriters {
    set_parameters: CommandWriter<DirectEffectParameters>,
    set_direction: CommandWriter<BinauralEffectParameters>,
}

pub(crate) struct CommandReaders {
    set_parameters: CommandReader<DirectEffectParameters>,
    set_direction: CommandReader<BinauralEffectParameters>,
}

pub(crate) fn command_writers_and_readers() -> (CommandWriters, CommandReaders) {
    let (set_parameters_writer, set_parameters_reader) = kira::command::command_writer_and_reader();
    let (set_direction_writer, set_direction_reader) = kira::command::command_writer_and_reader();

    let command_writers = CommandWriters {
        set_parameters: set_parameters_writer,
        set_direction: set_direction_writer,
    };

    let command_readers = CommandReaders {
        set_parameters: set_parameters_reader,
        set_direction: set_direction_reader,
    };

    (command_writers, command_readers)
}
