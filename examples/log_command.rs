use bevy::prelude::*;
use bevy_console::{reply, AddConsoleCommand, ConsoleCommand, ConsolePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ConsolePlugin)
        .add_console_command::<LogCommand, _, _>(log_command)
        .run();
}

/// Prints given arguments to the console
#[derive(ConsoleCommand)]
#[console_command(name = "log")]
struct LogCommand {
    /// Message to print
    msg: String,
    /// Number of times to print message
    num: Option<i64>,
}

fn log_command(mut log: ConsoleCommand<LogCommand>) {
    if let Some(LogCommand { msg, num }) = log.take() {
        let repeat_count = num.unwrap_or(1);

        for _ in 0..repeat_count {
            reply!(log, "{msg}");
        }

        log.ok();
    }
}
