use bevy::prelude::*;
use bevy_console::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(ConsolePlugin)
        .insert_resource(ConsoleConfiguration {
            key: ToggleConsoleKey::ScanCode(41), // this is the console key on a swedish keyboard
            ..Default::default()
        })
        .run();
}
