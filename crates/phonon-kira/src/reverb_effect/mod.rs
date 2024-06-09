use kira::command::{CommandReader, CommandWriter};

pub mod builder;
pub(crate) mod effect;
pub mod handle;

pub(crate) struct CommandWriters {
    set_reverb_times: CommandWriter<[f32; 3]>,
    set_wet: CommandWriter<bool>,
    set_dry: CommandWriter<bool>,
}

pub(crate) struct CommandReaders {
    set_reverb_times: CommandReader<[f32; 3]>,
    set_wet: CommandReader<bool>,
    set_dry: CommandReader<bool>,
}

pub(crate) fn command_writers_and_readers() -> (CommandWriters, CommandReaders) {
    let (set_reverb_times_writer, set_reverb_times_reader) =
        kira::command::command_writer_and_reader();
    let (set_wet_writer, set_wet_reader) = kira::command::command_writer_and_reader();
    let (set_dry_writer, set_dry_reader) = kira::command::command_writer_and_reader();

    let command_writers = CommandWriters {
        set_reverb_times: set_reverb_times_writer,
        set_wet: set_wet_writer,
        set_dry: set_dry_writer,
    };

    let command_readers = CommandReaders {
        set_reverb_times: set_reverb_times_reader,
        set_wet: set_wet_reader,
        set_dry: set_dry_reader,
    };

    (command_writers, command_readers)
}
