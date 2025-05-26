use bevy::log::LogPlugin;
use bevy::{log, prelude::*};
use bevy_console::{make_layer, ConsolePlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(LogPlugin {
                level: log::Level::INFO,
                filter: "error,capture_bevy_logs=info".to_owned(),
                custom_layer: make_layer,
            }),
            ConsolePlugin,
        ))
        .add_systems(Startup, || {
            log::info!("Hi!");
            log::warn!("This is a warning!");
            log::debug!("You won't see me!");
            log::error!("This is an error!");
            log::info!("Bye!");
        })
        .run();
}
