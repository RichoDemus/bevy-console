// pub use parse::parse_str;
use nom_supreme::{error::ErrorTree, final_parser::final_parser};
pub use value::{Value, ValueRaw, ValueRawOwned};

mod parse;
mod value;

pub type Error<I> = ErrorTree<I>;

#[derive(Clone, Debug, PartialEq)]
pub struct ConsoleCommand<'a> {
    pub command: &'a str,
    pub args: Vec<ValueRaw<'a>>,
}

pub fn parse_arg_str(s: &str) -> Result<Vec<ValueRaw>, nom::error::Error<&str>> {
    final_parser(parse::parse_value_list)(s)
}

pub fn parse_console_command(s: &str) -> Result<ConsoleCommand, nom::error::Error<&str>> {
    let (command, args) = final_parser(parse::parse_full_command)(s)?;

    Ok(ConsoleCommand { command, args })
}
