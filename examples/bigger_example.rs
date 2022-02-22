use bevy::{
    app::AppExit,
    ecs::system::{FunctionSystem, IsFunctionSystem},
    prelude::*,
};
use bevy_console::*;

fn main() {
    App::new()
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
        .add_startup_system(setup)
        .add_console_command("move_rect", move_rect_console_command)
        // .add_system(listen_to_console_events)
        .run();
}

#[derive(Component)]
struct Rect;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(-600.0, 300.0, 0.0).with_scale([10.0, 10.0, 0.0].into()),
            sprite: Sprite {
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Rect);
}

// fn move_rect_console_command(
//     direction: String,
//     amount: f64,
// ) -> impl for<'a> IntoSystem<(), (), (IsFunctionSystem, (Res<'a, Time>), ())> {
//     // FunctionSystem<(), (), (Res<'a, Time>,), (), fn(Res<'a, Time>)> {
//     // let p = handle_move_rest_console_command.system();
//     // p
//     move |time: Res<Time>| {
//         // time.delta_seconds();
//         // "hi".to_string()
//         // let mut transform = rect.single_mut();
//         // match direction.as_str() {
//         //     "left" => {
//         //         transform.translation.x -= 30.0;
//         //         format!("moved left by {:.2}px", amount)
//         //     }
//         //     "right" => {
//         //         transform.translation.x += 30.0;
//         //         format!("moved right by {:.2}px", amount)
//         //     }
//         //     "up" => {
//         //         transform.translation.y += 30.0;
//         //         format!("moved up by {:.2}px", amount)
//         //     }
//         //     "down" => {
//         //         transform.translation.y -= 30.0;
//         //         format!("moved down by {:.2}px", amount)
//         //     }
//         //     _ => "unknown direction".to_string(),
//         // }
//     }
// }

// fn handle_move_rest_console_command(time: Res<Time>) {}

// // fn listen_to_console_events(
// //     mut events: EventReader<ConsoleCommandEntered>,
// //     mut console_line: EventWriter<PrintConsoleLine>,
// //     mut app_exit_events: EventWriter<AppExit>,
// //     mut rect: Query<&mut Transform, With<Rect>>,
// // ) {
// //     for event in events.iter() {
// //         let event: &ConsoleCommandEntered = event;
// //         info!("Commands: {:?}", event);
// //         match event.command.as_str() {
// //             "move_rect" => {
// //                 let mov = match event.args.as_str() {
// //                     "left" => Vec3::new(-30., 0., 0.),
// //                     "up" => Vec3::new(0., 30., 0.),
// //                     "down" => Vec3::new(0., -30., 0.),
// //                     "right" => Vec3::new(30., 0., 0.),
// //                     _ => continue,
// //                 };
// //                 if let Ok(mut transform) = rect.single_mut() {
// //                     transform.translation += mov;
// //                 }
// //             }
// //             "quit" => {
// //                 app_exit_events.send(AppExit);
// //             }
// //             _ => continue, // unknown command
// //         }
// //         console_line.send(PrintConsoleLine::new("Ok".to_string()));
// //     }
// // }
