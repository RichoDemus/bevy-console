use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while_m_n},
    character::complete::{alpha1, alphanumeric1, char, multispace1, one_of, space0, space1},
    combinator::{map, map_opt, map_res, opt, recognize, value, verify},
    multi::{fold_many0, many0, many1, separated_list0},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};

use crate::ValueRaw;

/// Parse a unicode sequence, of the form u{XXXX}, where XXXX is 1 to 6
/// hexadecimal numerals. We will combine this later with parse_escaped_char
/// to parse sequences like \u{00AC}.
fn parse_unicode(input: &str) -> IResult<&str, char> {
    // `take_while_m_n` parses between `m` and `n` bytes (inclusive) that match
    // a predicate. `parse_hex` here parses between 1 and 6 hexadecimal numerals.
    let parse_hex = take_while_m_n(1, 6, |c: char| c.is_ascii_hexdigit());

    // `preceded` takes a prefix parser, and if it succeeds, returns the result
    // of the body parser. In this case, it parses u{XXXX}.
    let parse_delimited_hex = preceded(
        char('u'),
        // `delimited` is like `preceded`, but it parses both a prefix and a suffix.
        // It returns the result of the middle parser. In this case, it parses
        // {XXXX}, where XXXX is 1 to 6 hex numerals, and returns XXXX
        delimited(char('{'), parse_hex, char('}')),
    );

    // `map_res` takes the result of a parser and applies a function that returns
    // a Result. In this case we take the hex bytes from parse_hex and attempt to
    // convert them to a u32.
    let parse_u32 = map_res(parse_delimited_hex, move |hex| u32::from_str_radix(hex, 16));

    // map_opt is like map_res, but it takes an Option instead of a Result. If
    // the function returns None, map_opt returns an error. In this case, because
    // not all u32 values are valid unicode code points, we have to fallibly
    // convert to char with from_u32.
    map_opt(parse_u32, std::char::from_u32)(input)
}

/// Parse an escaped character: \n, \t, \r, \u{00AC}, etc.
fn parse_escaped_char(input: &str) -> IResult<&str, char> {
    preceded(
        char('\\'),
        // `alt` tries each parser in sequence, returning the result of
        // the first successful match
        alt((
            parse_unicode,
            // The `value` parser returns a fixed value (the first argument) if its
            // parser (the second argument) succeeds. In these cases, it looks for
            // the marker characters (n, r, t, etc) and returns the matching
            // character (\n, \r, \t, etc).
            value('\n', char('n')),
            value('\r', char('r')),
            value('\t', char('t')),
            value('\u{08}', char('b')),
            value('\u{0C}', char('f')),
            value('\\', char('\\')),
            value('/', char('/')),
            value('"', char('"')),
            value('\'', char('\'')),
        )),
    )(input)
}

/// Parse a backslash, followed by any amount of whitespace. This is used later
/// to discard any escaped whitespace.
fn parse_escaped_whitespace(input: &str) -> IResult<&str, &str> {
    preceded(char('\\'), multispace1)(input)
}

/// Parse a non-empty block of text that doesn't include \ or "
fn parse_literal(input: &str) -> IResult<&str, &str> {
    // `is_not` parses a string of 0 or more characters that aren't one of the
    // given characters.
    let not_quote_slash = is_not("\"\\\'");

    // `verify` runs a parser, then runs a verification function on the output of
    // the parser. The verification function accepts out output only if it
    // returns true. In this case, we want to ensure that the output of is_not
    // is non-empty.
    verify(not_quote_slash, |s: &str| !s.is_empty())(input)
}

/// A string fragment contains a fragment of a string being parsed: either
/// a non-empty Literal (a series of non-escaped characters), a single
/// parsed escaped character, or a block of escaped whitespace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StringFragment<'a> {
    Literal(&'a str),
    EscapedChar(char),
    EscapedWS,
}

/// Combine parse_literal, parse_escaped_whitespace, and parse_escaped_char
/// into a StringFragment.
fn parse_fragment<'a>(input: &'a str) -> IResult<&str, StringFragment<'a>> {
    alt((
        // The `map` combinator runs a parser, then applies a function to the output
        // of that parser.
        map(parse_literal, StringFragment::Literal),
        map(parse_escaped_char, StringFragment::EscapedChar),
        value(StringFragment::EscapedWS, parse_escaped_whitespace),
    ))(input)
}

/// Parse a string. Use a loop of parse_fragment and push all of the fragments
/// into an output string.
fn parse_string(input: &str) -> IResult<&str, String> {
    // fold_many0 is the equivalent of iterator::fold. It runs a parser in a loop,
    // and for each output value, calls a folding function on each output value.
    let build_string = || {
        fold_many0(
            // Our parser function– parses a single string fragment
            parse_fragment,
            // Our init value, an empty string
            String::new,
            // Our folding function. For each fragment, append the fragment to the
            // string.
            |mut string, fragment| {
                match fragment {
                    StringFragment::Literal(s) => string.push_str(s),
                    StringFragment::EscapedChar(c) => string.push(c),
                    StringFragment::EscapedWS => {}
                }
                string
            },
        )
    };

    // Finally, parse the string. Note that, if `build_string` could accept a raw
    // " character, the closing delimiter " would never match. When using
    // `delimited` with a looping parser (like fold_many0), be sure that the
    // loop won't accidentally match your closing delimiter!
    alt((
        delimited(char('"'), build_string(), char('"')),
        delimited(char('\''), build_string(), char('\'')),
    ))(input)
}

fn parse_int(input: &str) -> IResult<&str, (i64, &str)> {
    map_res(
        recognize(many1(terminated(one_of("0123456789"), many0(char('_'))))),
        |s: &str| s.replace('_', "").parse::<i64>().map(|res| (res, s)),
    )(input)
}

fn parse_float(input: &str) -> IResult<&str, (f64, &str)> {
    map_res(
        alt((
            // Case one: .42
            recognize(tuple((
                char('.'),
                parse_int,
                opt(tuple((one_of("eE"), opt(one_of("+-")), parse_int))),
            ))),
            // Case two: 42e42 and 42.42e42
            recognize(tuple((
                parse_int,
                opt(preceded(char('.'), parse_int)),
                one_of("eE"),
                opt(one_of("+-")),
                parse_int,
            ))),
            // Case three: 42. and 42.42
            recognize(tuple((parse_int, char('.'), opt(parse_int)))),
            // Cae four: basic int
            // recognize(parse_int),
        )),
        |s: &str| s.replace('_', "").parse::<f64>().map(|res| (res, s)),
    )(input)
}

fn parse_bool(input: &str) -> IResult<&str, (bool, &str)> {
    alt((
        map(alt((tag("true"), tag("TRUE"), tag("1"))), |s| (true, s)),
        map(alt((tag("false"), tag("FALSE"), tag("0"))), |s| (false, s)),
    ))(input)
}

fn parse_value(input: &str) -> IResult<&str, ValueRaw> {
    alt((
        map(parse_string, ValueRaw::String),
        map(parse_float, |(num, raw)| ValueRaw::Float(num, raw)),
        map(parse_int, |(num, raw)| ValueRaw::Int(num, raw)),
        map(parse_bool, |(b, raw)| ValueRaw::Bool(b, raw)),
        map(
            recognize(many1(one_of(
                "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOP_-",
            ))),
            |s: &str| ValueRaw::String(s.to_string()),
        ),
    ))(input)
}

pub fn parse_value_list(input: &str) -> IResult<&str, Vec<ValueRaw>> {
    delimited(space0, separated_list0(space1, parse_value), space0)(input)
}

pub fn parse_command_name(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ))(input)
}

pub fn parse_full_command(input: &str) -> IResult<&str, (&str, Vec<ValueRaw>)> {
    delimited(
        space0,
        alt((
            separated_pair(parse_command_name, space1, parse_value_list),
            map(parse_command_name, |command| (command, Vec::new())),
        )),
        space0,
    )(input)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::ValueRaw;

    use super::{parse_bool, parse_float, parse_int, parse_string, parse_value, parse_value_list};

    #[test]
    fn it_parses_strings() {
        assert_eq!(
            parse_string(r#""hello world""#),
            Ok(("", "hello world".to_string()))
        );
        assert_eq!(
            parse_string(r#""hello \"world""#),
            Ok(("", "hello \"world".to_string()))
        );
        assert_eq!(
            parse_string("'hello world'"),
            Ok(("", "hello world".to_string()))
        );
        assert!(parse_string(r#""hello world"#).is_err());
        assert!(parse_string("'hello world").is_err());
        assert!(parse_string(r#""hello world'"#).is_err());
        assert!(parse_string(r#"'hello world""#).is_err());
    }

    #[test]
    fn it_parses_ints() {
        assert_eq!(parse_int("124"), Ok(("", (124, "124"))));
        assert_eq!(parse_int("124hello"), Ok(("hello", (124, "124"))));
        assert_eq!(parse_int("123_456"), Ok(("", (123456, "123_456"))));
    }

    #[test]
    fn it_parses_floats() {
        assert_eq!(parse_float("124."), Ok(("", (124.0, "124."))));
        assert_eq!(parse_float("124.hello"), Ok(("hello", (124.0, "124."))));
        assert_eq!(parse_float("123_456."), Ok(("", (123456.0, "123_456."))));
        assert_eq!(
            parse_float("123_456.789_123"),
            Ok(("", (123456.789123, "123_456.789_123")))
        );
    }

    #[test]
    fn it_parses_bools() {
        assert_eq!(parse_bool("true"), Ok(("", (true, "true"))));
        assert_eq!(parse_bool("TRUE"), Ok(("", (true, "TRUE"))));
        assert_eq!(parse_bool("1"), Ok(("", (true, "1"))));
        assert_eq!(parse_bool("false"), Ok(("", (false, "false"))));
        assert_eq!(parse_bool("FALSE"), Ok(("", (false, "FALSE"))));
        assert_eq!(parse_bool("0"), Ok(("", (false, "0"))));
        assert_eq!(parse_bool("true_"), Ok(("_", (true, "true"))));
        assert_eq!(parse_bool("false_"), Ok(("_", (false, "false"))));
        assert_eq!(parse_bool("1_"), Ok(("_", (true, "1"))));
        assert_eq!(parse_bool("0_"), Ok(("_", (false, "0"))));
    }

    #[test]
    fn it_parses_values() {
        assert_eq!(
            parse_value(r#""hello world""#),
            Ok(("", ValueRaw::String("hello world".to_string())))
        );
        assert_eq!(
            parse_value(r#""hello world""#),
            Ok(("", ValueRaw::String("hello world".to_string())))
        );
    }

    #[test]
    fn it_parses_value_list() {
        assert_eq!(
            parse_value_list(r#""hello world" 10 true"#),
            Ok((
                "",
                vec![
                    ValueRaw::String("hello world".to_string()),
                    ValueRaw::Int(10, "10"),
                    ValueRaw::Bool(true, "true")
                ]
            ))
        );
        assert_eq!(
            parse_value_list(r#""hello world" 10. true"#),
            Ok((
                "",
                vec![
                    ValueRaw::String("hello world".to_string()),
                    ValueRaw::Float(10.0, "10."),
                    ValueRaw::Bool(true, "true")
                ]
            ))
        );
    }
}
