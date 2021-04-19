use bevy::app::AppExit;
use bevy::prelude::*;

use bevy_console::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(ConsolePlugin)
        .insert_resource(ConsoleConfiguration {
            help: vec![
                HelpCommand::new(
                    "move_rect".to_string(),
                    "Usage: move_rect <up/down/left/right>".to_string(),
                ),
                HelpCommand::new("quit".to_string(), "quits the app".to_string()),
            ],
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_system(listen_to_console_events.system())
        .run();
}

struct MyRect;

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_xyz(-600., 300., 0.),
            sprite: Sprite::new(Vec2::new(10., 10.)),
            ..Default::default()
        })
        .insert(MyRect);
}

/// listens to `ConsoleCommandEntered` events
/// moves rect or quits based on events
fn listen_to_console_events(
    mut events: EventReader<ConsoleCommandEntered>,
    mut console_line: EventWriter<PrintConsoleLine>,
    mut app_exit_events: EventWriter<AppExit>,
    mut rect: Query<&mut Transform, With<MyRect>>,
) {
    for event in events.iter() {
        let event: &ConsoleCommandEntered = event;
        info!("Commands: {:?}", event);
        match event.command.as_str() {
            "move_rect" => {
                let mov = match event.args.as_str() {
                    "left" => Vec3::new(-30., 0., 0.),
                    "up" => Vec3::new(0., 30., 0.),
                    "down" => Vec3::new(0., -30., 0.),
                    "right" => Vec3::new(30., 0., 0.),
                    _ => continue,
                };
                if let Ok(mut transform) = rect.single_mut() {
                    transform.translation += mov;
                }
            }
            "quit" => {
                app_exit_events.send(AppExit);
            }
            _ => continue, // unknown command
        }
        console_line.send(PrintConsoleLine::new("Ok".to_string()));
    }
}
