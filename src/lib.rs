#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use bevy::prelude::*;
pub use bevy_console_derive::ConsoleCommand;
pub use bevy_console_parser::{Value, ValueRawOwned};
use bevy_egui::EguiPlugin;
use commands::exit::{exit_command, ExitCommand};
use commands::help::{help_command, HelpCommand};

use crate::console::{console_system, receive_console_line, ConsoleState};
pub use crate::console::{
    AddConsoleCommand, CommandArgInfo, CommandArgs, CommandHelp, CommandInfo, CommandName,
    ConsoleCommand, ConsoleCommandEntered, ConsoleConfiguration, PrintConsoleLine,
    ToggleConsoleKey,
};
pub use crate::value::{FromValue, FromValueError, ValueType};

mod commands;
mod console;
mod macros;
mod value;

/// Console plugin.
pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ConsoleConfiguration>()
            .insert_resource(ConsoleState::default())
            .add_event::<ConsoleCommandEntered>()
            .add_event::<PrintConsoleLine>()
            .add_plugin(EguiPlugin)
            .add_console_command::<ExitCommand, _, _>(exit_command)
            .add_console_command::<HelpCommand, _, _>(help_command)
            // if there's other egui code we need to make sure they don't run at the same time
            .add_system(console_system.exclusive_system())
            .add_system(receive_console_line);
    }
}
