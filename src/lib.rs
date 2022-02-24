#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use bevy::prelude::*;
pub use bevy_console_derive::ConsoleCommand;
pub use bevy_console_parser::{Value, ValueRawOwned};
use bevy_egui::EguiPlugin;

use crate::commands::clear::{clear_command, ClearCommand};
use crate::commands::exit::{exit_command, ExitCommand};
use crate::commands::help::{help_command, HelpCommand};
use crate::console::{console_toggle, receive_console_line, ConsoleOpen, ConsoleState};
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
            .init_resource::<ConsoleState>()
            .init_resource::<ConsoleOpen>()
            .add_event::<ConsoleCommandEntered>()
            .add_event::<PrintConsoleLine>()
            .add_plugin(EguiPlugin)
            .add_console_command::<ClearCommand, _, _>(clear_command)
            .add_console_command::<ExitCommand, _, _>(exit_command)
            .add_console_command::<HelpCommand, _, _>(help_command)
            .add_system(console_toggle.exclusive_system())
            .add_system(receive_console_line);
    }
}
