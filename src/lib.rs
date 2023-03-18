#![doc = include_str ! ("../README.md")]
#![deny(missing_docs)]

use bevy::prelude::*;
pub use bevy_console_derive::ConsoleCommand;
use bevy_egui::{EguiPlugin, EguiSettings};

use crate::commands::clear::{clear_command, ClearCommand};
use crate::commands::exit::{exit_command, ExitCommand};
use crate::commands::help::{help_command, HelpCommand};
pub use crate::console::{
    AddConsoleCommand, Command, ConsoleCommand, ConsoleCommandEntered, ConsoleConfiguration,
    ConsoleOpen, NamedCommand, PrintConsoleLine, ToggleConsoleKey,
};
// pub use color::{Style, StyledStr};

use crate::console::{console_ui, receive_console_line, ConsoleState};

// mod color;
mod commands;
mod console;
mod macros;

/// Console plugin.
pub struct ConsolePlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
/// The SystemSet for console/command related systems
pub enum ConsoleSet {
    /// Systems operating the console UI (the input layer)
    ConsoleUI,

    /// Systems executing console commands (the functionality layer).
    /// All command handler systems are added to this set
    Commands,

    /// Systems running after command systems, which depend on the fact commands have executed beforehand (the output layer).
    /// For example a system which makes use of [`PrintConsoleLine`] events should be placed in this set to be able to receive
    /// New lines to print in the same frame
    PostCommands,
}

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ConsoleConfiguration>()
            .init_resource::<ConsoleState>()
            .init_resource::<ConsoleOpen>()
            .add_event::<ConsoleCommandEntered>()
            .add_event::<PrintConsoleLine>()
            .add_console_command::<ClearCommand, _>(clear_command)
            .add_console_command::<ExitCommand, _>(exit_command)
            .add_console_command::<HelpCommand, _>(help_command)
            .add_system(console_ui.in_set(ConsoleSet::ConsoleUI))
            .add_system(receive_console_line.in_set(ConsoleSet::PostCommands))
            .configure_set(ConsoleSet::Commands.after(ConsoleSet::ConsoleUI))
            .configure_set(ConsoleSet::PostCommands.after(ConsoleSet::Commands));

        // Don't initialize an egui plugin if one already exists.
        // This can happen if another plugin is using egui and was installed before us.
        if !app.world.contains_resource::<EguiSettings>() {
            app.add_plugin(EguiPlugin);
        }
    }
}
