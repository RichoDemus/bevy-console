use bevy::prelude::*;
use bevy_console::{ConsoleConfiguration, ConsolePlugin, ToggleConsoleKey};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ConsolePlugin))
        .insert_resource(ConsoleConfiguration {
            keys: vec![
                ToggleConsoleKey::ScanCode(41), // Console key on a swedish keyboard
                ToggleConsoleKey::KeyCode(KeyCode::Grave), // US console key
                ToggleConsoleKey::KeyCode(KeyCode::F1),
            ],
            ..Default::default()
        })
        .run();
}
