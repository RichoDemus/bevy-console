use bevy::prelude::*;

use crate as bevy_console;
use crate::console::ConsoleState;
use crate::{ClapConsoleCommand, ConsoleCommand};

/// Clears the console
#[derive(Resource, clap::Parser)]
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

use clap::Parser;

#[derive(Parser)]
#[command(author,version,about,long_about = None)]
pub(crate) struct DummyCommand;
