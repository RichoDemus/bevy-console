use std::fmt;

use bevy::prelude::IntoSystem;
use bevy_console_parser::{Value, ValueRawOwned};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ValueType {
    String,
    Int,
    Float,
    Bool,
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueType::String => write!(f, "string"),
            ValueType::Int => write!(f, "int"),
            ValueType::Float => write!(f, "float"),
            ValueType::Bool => write!(f, "bool"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum FromValueError {
    NotEnoughArgs,
    UnexpectedArgType {
        arg_num: u8,
        expected: ValueType,
        received: ValueType,
    },
    Custom(String),
}

impl fmt::Display for FromValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FromValueError::NotEnoughArgs => write!(f, "[error] not enough arguments provided"),
            FromValueError::UnexpectedArgType {
                arg_num,
                expected,
                received,
            } => write!(
                f,
                "[error] expected '{expected}' but got '{received}' for arg {}",
                arg_num + 1
            ),
            FromValueError::Custom(msg) => write!(f, "[error] {msg}"),
        }
    }
}

pub trait FromValue<'a>: Sized {
    fn from_value(value: &'a ValueRawOwned, arg_num: u8) -> Result<Self, FromValueError>;

    fn from_value_iter<I>(value: &mut I, arg_num: u8) -> Result<Self, FromValueError>
    where
        I: Iterator<Item = &'a ValueRawOwned>,
    {
        Self::from_value(value.next().ok_or(FromValueError::NotEnoughArgs)?, arg_num)
    }
}

impl<'a> FromValue<'a> for Value {
    fn from_value(value: &'a ValueRawOwned, _arg_num: u8) -> Result<Self, FromValueError> {
        Ok(value.clone().into())
    }
}

impl<'a> FromValue<'a> for ValueRawOwned {
    fn from_value(value: &'a ValueRawOwned, _arg_num: u8) -> Result<Self, FromValueError> {
        Ok(value.clone())
    }
}

use crate::IntoCommandResult;

macro_rules! unexpected_arg_type {
    ($expected: ident, $received: ident, $arg_num: ident) => {
        FromValueError::UnexpectedArgType {
            arg_num: $arg_num,
            expected: ValueType::$expected,
            received: ValueType::$received,
        }
    };
}

impl FromValue<'_> for String {
    fn from_value(value: &ValueRawOwned, _arg_num: u8) -> Result<Self, FromValueError> {
        match value {
            ValueRawOwned::String(s) => Ok(s.clone()),
            ValueRawOwned::Int(_, raw)
            | ValueRawOwned::Float(_, raw)
            | ValueRawOwned::Bool(_, raw) => Ok(raw.to_string()),
        }
    }
}

impl FromValue<'_> for i64 {
    fn from_value(value: &ValueRawOwned, arg_num: u8) -> Result<Self, FromValueError> {
        match value {
            ValueRawOwned::String(_) => Err(unexpected_arg_type!(Int, String, arg_num)),
            ValueRawOwned::Int(num, _) => Ok(*num),
            ValueRawOwned::Float(_, _) => Err(unexpected_arg_type!(Int, Float, arg_num)),
            ValueRawOwned::Bool(_, _) => Err(unexpected_arg_type!(Int, Bool, arg_num)),
        }
    }
}

impl FromValue<'_> for f64 {
    fn from_value(value: &ValueRawOwned, arg_num: u8) -> Result<Self, FromValueError> {
        match value {
            ValueRawOwned::String(_) => Err(unexpected_arg_type!(Float, String, arg_num)),
            ValueRawOwned::Int(num, _) => Ok(*num as f64),
            ValueRawOwned::Float(num, _) => Ok(*num),
            ValueRawOwned::Bool(_, _) => Err(unexpected_arg_type!(Float, Bool, arg_num)),
        }
    }
}

impl FromValue<'_> for bool {
    fn from_value(value: &ValueRawOwned, arg_num: u8) -> Result<Self, FromValueError> {
        match value {
            ValueRawOwned::String(_) => Err(unexpected_arg_type!(Bool, String, arg_num)),
            ValueRawOwned::Int(_, _) => Err(unexpected_arg_type!(Bool, Int, arg_num)),
            ValueRawOwned::Float(_, _) => Err(unexpected_arg_type!(Bool, Float, arg_num)),
            ValueRawOwned::Bool(b, _) => Ok(*b),
        }
    }
}

impl<'a, T> FromValue<'a> for Option<T>
where
    T: FromValue<'a>,
{
    fn from_value(value: &'a ValueRawOwned, arg_num: u8) -> Result<Self, FromValueError> {
        Ok(Some(T::from_value(value, arg_num)?))
    }

    fn from_value_iter<I>(value: &mut I, arg_num: u8) -> Result<Self, FromValueError>
    where
        I: Iterator<Item = &'a ValueRawOwned>,
    {
        value
            .next()
            .map(|value| T::from_value(value, arg_num))
            .transpose()
    }
}

pub trait RunWithValues<Sys, Params, In, Out, P>
where
    Sys: IntoSystem<In, Out, P>,
    Out: IntoCommandResult,
{
    fn run_with_values(&self, values: &[ValueRawOwned]) -> Result<Sys, FromValueError>;
}

macro_rules! impl_run_with_values {
    ($( $i: ident ),*) => {
        impl<Func, Sys, In, Out, P, $($i),*> RunWithValues<Sys, ($($i,)*), In, Out, P> for Func
        where
            Func: 'static + Send + Sync + Fn($($i,)*) -> Sys,
            Sys: IntoSystem<In, Out, P>,
            Out: IntoCommandResult,
            $($i: for<'a> FromValue<'a>),*
        {
            #[allow(unused_variables, unused_mut, unused_assignments)]
            fn run_with_values(&self, values: &[ValueRawOwned]) -> Result<Sys, FromValueError> {
                let mut values_iter = values.iter();
                let mut arg_n = 0;
                $(
                    #[allow(non_snake_case)]
                    let $i = <$i as FromValue>::from_value_iter(&mut values_iter, arg_n)?;
                    arg_n += 1;
                )*

                Ok(self($($i, )*))
            }
        }
    };
}

// Stairway to bevyn
impl_run_with_values!();
impl_run_with_values!(F0);
impl_run_with_values!(F0, F1);
impl_run_with_values!(F0, F1, F2);
impl_run_with_values!(F0, F1, F2, F3);
impl_run_with_values!(F0, F1, F2, F3, F4);
impl_run_with_values!(F0, F1, F2, F3, F4, F5);
impl_run_with_values!(F0, F1, F2, F3, F4, F5, F6);
impl_run_with_values!(F0, F1, F2, F3, F4, F5, F6, F7);
impl_run_with_values!(F0, F1, F2, F3, F4, F5, F6, F7, F8);
impl_run_with_values!(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9);
impl_run_with_values!(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10);
impl_run_with_values!(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11);
impl_run_with_values!(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12);
impl_run_with_values!(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13);
impl_run_with_values!(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14);
impl_run_with_values!(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15);
impl_run_with_values!(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16);
