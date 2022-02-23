use bevy::prelude::*;
use bevy_console::{ConsolePlugin, PrintConsoleLine};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ConsolePlugin)
        .add_system(write_to_console)
        .run();
}

fn write_to_console(mut console_line: EventWriter<PrintConsoleLine>) {
    console_line.send(PrintConsoleLine::new("Hello".to_string()));
}
