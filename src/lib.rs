use bevy::prelude::*;
pub use bevy_console_parser::{Value, ValueRawOwned};
use bevy_egui::EguiPlugin;

pub use crate::command_result::{CommandResult, IntoCommandResult};
use crate::console::{console_system, receive_console_line, ConsoleState};
pub use crate::console::{
    CommandArgs, CommandName, ConsoleCommand, ConsoleCommandEntered, ConsoleConfiguration,
    HelpCommand, PrintConsoleLine, ToggleConsoleKey,
};
pub use crate::value::{FromValue, FromValueError, RunWithValues, ValueType};

mod command_result;
mod console;
mod value;

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ConsoleState::default())
            .add_event::<ConsoleCommandEntered>()
            .add_event::<PrintConsoleLine>()
            .add_plugin(EguiPlugin)
            // if there's other egui code we need to make sure they don't run at the same time
            .add_startup_system(setup_cached_console_commands_system_state.exclusive_system())
            .add_system(console_system.exclusive_system())
            .add_system(receive_console_line);
    }
}

#[macro_export]
macro_rules! reply {
    ($cmd: ident, $fmt: literal$(, $($arg:expr),* $(,)?)?) => {
        {
            let msg = format!($fmt$(, $($arg),*)?);
            $cmd.reply(msg);
        }
    };
}

#[macro_export]
macro_rules! reply_ok {
    ($cmd: ident, $fmt: literal$(, $($arg:expr),* $(,)?)?) => {
        {
            let msg = format!($fmt$(, $($arg),*)?);
            $cmd.reply(msg);
            $cmd.ok();
        }
    };
}

#[macro_export]
macro_rules! reply_fail {
    ($cmd: ident, $fmt: literal$(, $($arg:expr),* $(,)?)?) => {
        {
            let msg = format!($fmt$(, $($arg),*)?);
            $cmd.reply(msg);
            $cmd.fail();
        }
    };
}
