use bevy::prelude::*;
use bevy_console::{reply_ok, ConsoleCommand, ConsolePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ConsolePlugin)
        .add_system(log_command)
        .run();
}

#[derive(ConsoleCommand)]
#[console_command(name = "log")]
struct LogCommand {
    msg: String,
    name: String,
    age: Option<i64>,
}

fn log_command(mut log: ConsoleCommand<LogCommand>, time: Res<Time>) {
    if let Some(cmd) = log.single() {
        reply_ok!(
            log,
            "You said {} at {}, btw ur name is {} and age is {:?}",
            cmd.msg,
            time.seconds_since_startup(),
            cmd.name,
            cmd.age
        );
    }
}
