use bevy::prelude::*;
use bevy_console::{reply, AddConsoleCommand, ConsoleCommand, ConsoleConfiguration, ConsolePlugin};
use clap::{Parser, ValueEnum};

fn main() {
    App::new()
        // set background to red
        .add_plugins((DefaultPlugins, ConsolePlugin))
        .insert_resource(ConsoleConfiguration {
            arg_completions: vec![
                vec!["custom".into(), "foo".into()],
                vec!["custom".into(), "bar".into()],
                vec!["custom".into(), "zoo".into()],
            ],
            ..Default::default()
        })
        .add_console_command::<CustomCommand, _>(log_command)
        .run();
}

#[derive(Clone, Copy, ValueEnum)]
pub enum Variant {
    Foo,
    Bar,
    Zoo,
}

/// Prints given arguments to the console
#[derive(Parser, ConsoleCommand)]
#[command(name = "custom")]
struct CustomCommand {
    #[arg(value_enum)]
    variant: Variant,
}

fn log_command(mut log: ConsoleCommand<CustomCommand>) {
    if let Some(Ok(CustomCommand { variant })) = log.take() {
        match variant {
            Variant::Foo => reply!(log, "foo!"),
            Variant::Bar => reply!(log, "bar!"),
            Variant::Zoo => reply!(log, "zoo!"),
        }
        log.ok();
    }
}
