use bevy::prelude::*;
use bevy_console::{ConsolePlugin, ConsoleSet, PrintConsoleLine};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ConsolePlugin)
        .add_system(write_to_console.in_set(ConsoleSet::Commands))
        .run();
}

fn write_to_console(mut console_line: EventWriter<PrintConsoleLine>) {
    console_line.send(PrintConsoleLine::new("Hello".into()));
}
