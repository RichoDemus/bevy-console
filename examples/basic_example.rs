use bevy::prelude::*;
use bevy_console::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(ConsolePlugin)
        .run();
}
