use kira::command::{CommandReader, CommandWriter};
use phonon::effects::direct::DirectEffectParameters;
use phonon::effects::panning::PanningEffectParameters;

pub mod builder;
pub(crate) mod effect;
pub mod handle;

pub(crate) struct CommandWriters {
    set_parameters: CommandWriter<DirectEffectParameters>,
    set_panning: CommandWriter<PanningEffectParameters>,
}

pub(crate) struct CommandReaders {
    set_parameters: CommandReader<DirectEffectParameters>,
    set_panning: CommandReader<PanningEffectParameters>,
}

pub(crate) fn command_writers_and_readers() -> (CommandWriters, CommandReaders) {
    let (set_parameters_writer, set_parameters_reader) = kira::command::command_writer_and_reader();
    let (set_panning_writer, set_panning_reader) = kira::command::command_writer_and_reader();

    let command_writers = CommandWriters {
        set_parameters: set_parameters_writer,
        set_panning: set_panning_writer,
    };

    let command_readers = CommandReaders {
        set_parameters: set_parameters_reader,
        set_panning: set_panning_reader,
    };

    (command_writers, command_readers)
}
