use bevy::prelude::*;
use bevy_console::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ConsolePlugin)
        .insert_resource(ConsoleConfiguration {
            ..Default::default()
        })
        .run();
}
