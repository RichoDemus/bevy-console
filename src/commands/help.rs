use bevy::prelude::*;
use clap::Parser;

use crate as bevy_console;
use crate::{reply, ConsoleCommand, ConsoleConfiguration};

/// Prints available arguments and usage
#[derive(Parser, ConsoleCommand)]
#[command(name = "help")]
pub(crate) struct HelpCommand {
    /// Help for a given command
    command: Option<String>,
}

pub(crate) fn help_command(
    mut help: ConsoleCommand<HelpCommand>,
    mut config: ResMut<ConsoleConfiguration>,
) {
    match help.take() {
        Some(Ok(HelpCommand { command: Some(cmd) })) => match config.commands.get_mut(cmd.as_str())
        {
            Some(command_info) => {
                help.reply(command_info.command.render_long_help().to_string());
            }
            None => {
                reply!(help, "Command '{}' does not exist", cmd);
            }
        },
        Some(Ok(HelpCommand { command: None })) => {
            debug!("No command received in help");
            reply!(help, "Available commands:");
            let longest_command_name = config
                .commands
                .keys()
                .map(|name| name.len())
                .max()
                .unwrap_or(0);
            for (name, a_cmd) in &config.commands {
                let mut line = format!("  {name}{}", " ".repeat(longest_command_name - name.len()));
                line.push_str(&format!(
                    " - {}",
                    a_cmd
                        .command
                        .get_about()
                        .map(|about| about.to_string())
                        .unwrap_or_default()
                ));
                help.reply(line);
            }
            help.reply("");
        }
        _ => {}
    }
}
