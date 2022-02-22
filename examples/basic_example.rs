use std::marker::PhantomData;

use bevy::{
    ecs::{
        schedule::IntoSystemDescriptor,
        system::{
            IsFunctionSystem, Resource, SystemMeta, SystemParam, SystemParamFetch,
            SystemParamState, SystemState,
        },
    },
    prelude::*,
};
use bevy_console::{
    AddConsoleCommand, ConsoleCommandEntered, ConsoleConfiguration, ConsolePlugin, FromValue,
    FromValueError, PrintConsoleLine, ValueRawOwned,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ConsolePlugin)
        .insert_resource(ConsoleConfiguration::default())
        .add_system(log_command)
        // .add_console_command("log", log_command)
        .run();
}

pub struct LogCommand {
    msg: String,
}

impl CommandName for LogCommand {
    fn command_name() -> &'static str {
        "log"
    }
}

impl CommandArgs for LogCommand {
    fn from_values(values: &[ValueRawOwned]) -> Result<Self, FromValueError> {
        let mut values = values.iter();
        let msg = String::from_value_iter(&mut values, 0)?;

        Ok(LogCommand { msg })
    }
}

fn log_command(console_command: ConsoleCommand<LogCommand>) {}

// fn log_command(
//     msg: String,
//     // This can be switched to an impl type alias once it's stable
// ) -> impl IntoSystemDescriptor<()> {
//     move || {
//         format!("You said: {msg}");
//     }
// }

pub trait CommandName {
    fn command_name() -> &'static str;
}

pub trait CommandArgs: Sized {
    fn from_values(values: &[ValueRawOwned]) -> Result<Self, FromValueError>;
}

pub struct ConsoleCommand<'w, 's, T> {
    command: Option<T>,
    event_reader: EventReader<'w, 's, ConsoleCommandEntered>,
    console_line: EventWriter<'w, 's, PrintConsoleLine>,
}

impl<'w, 's, T: Resource + CommandName + CommandArgs> SystemParam for ConsoleCommand<'w, 's, T> {
    type Fetch = ConsoleCommandState<'w, 's, T>;
}

pub struct ConsoleCommandState<'w, 's, T> {
    system_state: SystemState<(
        EventReader<ConsoleCommandEntered>,
        EventWriter<PrintConsoleLine>,
    )>,
    // event_reader: EventReader<'w, 's, ConsoleCommandEntered>,
    // console_line: EventWriter<'w, 's, PrintConsoleLine>,
    marker: PhantomData<T>,
}

unsafe impl<'w, 's, T: Resource> SystemParamState for ConsoleCommandState<'w, 's, T> {
    type Config = ();

    fn init(world: &mut World, system_meta: &mut SystemMeta, _config: Self::Config) -> Self {
        world.query::<EventReader<ConsoleCommandEntered>>();

        let mut event_reader = world
            .get_resource::<EventReader<ConsoleCommandEntered>>()
            .unwrap_or_else(|| {
                panic!("EventReader<ConsoleCommandEntered> requested does not exist")
            });

        let console_line = world
            .get_resource_mut::<EventWriter<PrintConsoleLine>>()
            .unwrap_or_else(|| panic!("EventWriter<PrintConsoleLine> requested does not exist"));

        ConsoleCommandState {
            event_reader,
            console_line,
            marker: PhantomData::default(),
        }
    }

    fn default_config() {}
}

impl<'w, 's, T: Resource + CommandName + CommandArgs> SystemParamFetch<'w, 's>
    for ConsoleCommandState<'w, 's, T>
{
    type Item = ConsoleCommand<'w, 's, T>;

    #[inline]
    unsafe fn get_param(
        state: &'s mut Self,
        system_meta: &SystemMeta,
        world: &'w World,
        change_tick: u32,
    ) -> Self::Item {
        // let mut console_command_events = world
        //     .get_resource::<EventReader<ConsoleCommandEntered>>()
        //     .unwrap_or_else(|| {
        //         panic!("EventReader<ConsoleCommandEntered> requested does not exist")
        //     });

        // let command = console_command_events
        //     .iter()
        //     .find(|cmd| cmd.command == T::command_name())
        //     .map(|cmd| T::from_values(&cmd.args))
        //     .and_then(|result| match result {
        //         Ok(value) => Some(value),
        //         Err(err) => {
        //             let console_line = world
        //                 .get_resource_mut::<EventWriter<PrintConsoleLine>>()
        //                 .unwrap_or_else(|| {
        //                     panic!("EventWriter<PrintConsoleLine> requested does not exist")
        //                 });
        //             console_line.send(PrintConsoleLine::new(err.to_string()));
        //             None
        //         }
        //     });

        todo!()
    }
}
