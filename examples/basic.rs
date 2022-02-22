use bevy::prelude::*;
use bevy_console::{
    reply_ok, CommandArgs, CommandName, ConsoleCommand, ConsoleConfiguration, ConsolePlugin,
    FromValue, FromValueError, ValueRawOwned,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ConsolePlugin)
        .add_system(log_command)
        .run();
}

struct LogCommand {
    msg: String,
}

impl CommandName for LogCommand {
    fn command_name() -> &'static str {
        "log"
    }
}

impl CommandArgs for LogCommand {
    fn from_values(values: &[ValueRawOwned]) -> Result<Self, FromValueError> {
        let mut values = values.iter();
        let msg = String::from_value_iter(&mut values, 0)?;

        Ok(LogCommand { msg })
    }
}

fn log_command(mut log: ConsoleCommand<LogCommand>, time: Res<Time>) {
    if let Some(cmd) = log.single() {
        reply_ok!(
            log,
            "You said {} at {}",
            cmd.msg,
            time.seconds_since_startup()
        );
    }
}
