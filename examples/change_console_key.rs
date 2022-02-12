use bevy::prelude::*;
use bevy_console::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ConsolePlugin)
        .insert_resource(ConsoleConfiguration {
            keys: vec![
                ToggleConsoleKey::ScanCode(41), // this is the console key on a swedish keyboard
                ToggleConsoleKey::KeyCode(KeyCode::Grave), // US console key
            ],
            ..Default::default()
        })
        .run();
}
