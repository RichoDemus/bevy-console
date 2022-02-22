use std::marker::PhantomData;

use bevy::{
    app::{EventReaderState, EventWriterState, Events},
    ecs::system::{
        LocalState, ResMutState, ResState, Resource, SystemMeta, SystemParam, SystemParamFetch,
        SystemParamState,
    },
    prelude::*,
};
use bevy_console::{ConsoleCommandEntered, FromValueError, PrintConsoleLine, ValueRawOwned};

fn main() {}

pub trait CommandName {
    fn command_name() -> &'static str;
}

pub trait CommandArgs: Sized {
    fn from_values(values: &[ValueRawOwned]) -> Result<Self, FromValueError>;
}
