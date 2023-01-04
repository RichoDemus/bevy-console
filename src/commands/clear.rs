use bevy::prelude::*;

use crate as bevy_console;
use crate::console::ConsoleState;
use crate::ConsoleCommand;
use clap::Parser;

/// Clears the console
#[derive(Resource, Parser, ConsoleCommand)]
#[command(name = "clear")]
pub(crate) struct ClearCommand;

pub(crate) fn clear_command(
    mut clear: ConsoleCommand<ClearCommand>,
    mut state: ResMut<ConsoleState>,
) {
    if clear.take().is_some() {
        state.scrollback.clear();
    }
}
