use crate::ValueType;
use std::fmt;

/// Error when trying to parse values.
#[derive(Clone, Debug, PartialEq)]
pub enum FromValueError {
    /// Not enough arguments provided
    NotEnoughArgs,
    /// Unexpected argument type
    UnexpectedArgType {
        /// Argument number, starting from 0
        arg_num: u8,
        /// Expected value type
        expected: ValueType,
        /// Received value type
        received: ValueType,
    },
    /// Value too large
    ValueTooLarge {
        /// Argument number, starting from 0
        arg_num: u8,
        /// Maximum allowed value
        max: i64,
    },
    /// Custom error
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
                "[error] expected '{expected}' but got '{received}' for arg #{}",
                arg_num + 1
            ),
            FromValueError::ValueTooLarge { arg_num, max } => {
                write!(
                    f,
                    "[error] number is too large for arg #{} (max {})",
                    arg_num + 1,
                    max
                )
            }
            FromValueError::Custom(msg) => write!(f, "[error] {msg}"),
        }
    }
}
