use bevy::{
    ecs::{schedule::IntoSystemDescriptor, system::IsFunctionSystem},
    prelude::*,
};
use bevy_console::{ConsoleConfiguration, ConsolePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ConsolePlugin)
        .insert_resource(ConsoleConfiguration::default())
        .add_console_command("log", log_command)
        .add_system()
        .run();
}

pub trait AddConsoleCommand {
    fn add_console_command<Params>(
        &mut self,
        command: &'static str,
        handler: impl IntoSystemDescriptor<Params>,
    ) -> &mut Self
    where
        F: RunWithValues<Sys, Params, (), Out, P> + Send + Sync + 'static,
        Sys: IntoSystem<(), Out, P>,
        Out: IntoCommandResult;
}

impl AddConsoleCommand for App {
    fn add_console_command<F, Sys, Params, Out, P>(
        &mut self,
        command: &'static str,
        system: F,
    ) -> &mut Self
    where
        F: RunWithValues<Sys, Params, (), Out, P> + Send + Sync + 'static,
        Sys: IntoSystem<(), Out, P>,
        Out: IntoCommandResult,
    {
        app.add_system(system)
    }
}
pub struct ConsoleCommand<C, A>(C, A);

fn log_command(
    msg: String,
    // This can be switched to an impl type alias once it's stable
) -> impl IntoSystem<(), String, (IsFunctionSystem, (), ())> {
    move || format!("You said: {msg}")
}
