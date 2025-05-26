use std::collections::HashSet;

use ansi_parser::AnsiParser;
use bevy_egui::egui::Color32;

pub(crate) fn parse_ansi_styled_str(
    ansi_string: &str,
) -> Vec<(&str, HashSet<TextFormattingOverride>)> {
    let mut result: Vec<(&str, HashSet<TextFormattingOverride>)> = Vec::new();
    let mut current_overrides = HashSet::new();
    for element in ansi_string.ansi_parse() {
        match element {
            ansi_parser::Output::TextBlock(t) => {
                result.push((t, current_overrides.clone()));
            }
            ansi_parser::Output::Escape(escape) => {
                if let ansi_parser::AnsiSequence::SetGraphicsMode(mode) = escape {
                    let modes = parse_graphics_mode(mode.as_slice());
                    for m in modes.iter() {
                        apply_set_graphics_mode(&mut current_overrides, *m);
                    }
                }
            }
        }
    }
    result
}

fn apply_set_graphics_mode(
    set_overrides: &mut HashSet<TextFormattingOverride>,
    new: TextFormattingOverride,
) {
    match new {
        TextFormattingOverride::ResetEveryting => {
            set_overrides.clear();
        }
        TextFormattingOverride::ResetDimAndBold => {
            set_overrides.remove(&TextFormattingOverride::Dim);
            set_overrides.remove(&TextFormattingOverride::Bold);
        }
        TextFormattingOverride::ResetItalicsAndFraktur => {
            set_overrides.remove(&TextFormattingOverride::Italic);
        }
        TextFormattingOverride::ResetUnderline => {
            set_overrides.remove(&TextFormattingOverride::Underline);
        }
        TextFormattingOverride::ResetStrikethrough => {
            set_overrides.remove(&TextFormattingOverride::Strikethrough);
        }
        TextFormattingOverride::ResetForegroundColor => {
            set_overrides.retain(|o| !matches!(o, TextFormattingOverride::Foreground(_)));
        }
        TextFormattingOverride::ResetBackgroundColor => {
            set_overrides.retain(|o| !matches!(o, TextFormattingOverride::Background(_)));
        }
        _ => {
            set_overrides.insert(new);
        }
    }
}

fn parse_graphics_mode(modes: &[u8]) -> HashSet<TextFormattingOverride> {
    let mut results = HashSet::new();
    for mode in modes.iter() {
        let result = match *mode {
            1 => TextFormattingOverride::Bold,
            2 => TextFormattingOverride::Dim,
            3 => TextFormattingOverride::Italic,
            4 => TextFormattingOverride::Underline,
            9 => TextFormattingOverride::Strikethrough,
            22 => TextFormattingOverride::ResetDimAndBold,
            23 => TextFormattingOverride::ResetItalicsAndFraktur,
            24 => TextFormattingOverride::ResetUnderline,
            29 => TextFormattingOverride::ResetStrikethrough,
            30..=37 => TextFormattingOverride::Foreground(ansi_color_code_to_color32(mode - 30)),
            39 => TextFormattingOverride::ResetForegroundColor,
            40..=47 => TextFormattingOverride::Background(ansi_color_code_to_color32(mode - 40)),
            49 => TextFormattingOverride::ResetBackgroundColor,
            _ => TextFormattingOverride::ResetEveryting,
        };
        results.insert(result);
    }
    results
}

fn ansi_color_code_to_color32(color_code: u8) -> Color32 {
    match color_code {
        1 => Color32::from_rgb(222, 56, 43),    // red
        2 => Color32::from_rgb(57, 181, 74),    // green
        3 => Color32::from_rgb(255, 199, 6),    // yellow
        4 => Color32::from_rgb(0, 111, 184),    // blue
        5 => Color32::from_rgb(118, 38, 113),   // magenta
        6 => Color32::from_rgb(44, 181, 233),   // cyan
        7 => Color32::from_rgb(204, 204, 204),  // white
        8 => Color32::from_rgb(128, 128, 128),  // bright black
        9 => Color32::from_rgb(255, 0, 0),      // bright red
        10 => Color32::from_rgb(0, 255, 0),     // bright green
        11 => Color32::from_rgb(255, 255, 0),   // bright yellow
        12 => Color32::from_rgb(0, 0, 255),     // bright blue
        13 => Color32::from_rgb(255, 0, 255),   // bright magenta
        14 => Color32::from_rgb(0, 255, 255),   // bright cyan
        15 => Color32::from_rgb(255, 255, 255), // bright white
        _ => Color32::from_rgb(1, 1, 1),        // black
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum TextFormattingOverride {
    ResetEveryting,
    ResetDimAndBold,
    ResetItalicsAndFraktur,
    ResetUnderline,
    ResetStrikethrough,
    ResetForegroundColor,
    ResetBackgroundColor,
    Bold,
    Dim,
    Italic,
    Underline,
    Strikethrough,
    Foreground(Color32),
    Background(Color32),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bold_text() {
        let ansi_string = color_print::cstr!(r#"<bold>12345</bold>"#);
        let result = parse_ansi_styled_str(ansi_string);
        assert_eq!(
            result,
            vec![("12345", HashSet::from([TextFormattingOverride::Bold])),]
        );
    }

    #[test]
    fn test_underlined_text() {
        let ansi_string = color_print::cstr!(r#"<underline>12345</underline>"#);
        let result = parse_ansi_styled_str(ansi_string);
        assert_eq!(
            result,
            vec![("12345", HashSet::from([TextFormattingOverride::Underline])),]
        );
    }

    #[test]
    fn test_italics_text() {
        let ansi_string = color_print::cstr!(r#"<italic>12345</italic>"#);
        let result = parse_ansi_styled_str(ansi_string);
        assert_eq!(
            result,
            vec![("12345", HashSet::from([TextFormattingOverride::Italic])),]
        );
    }

    #[test]
    fn test_dim_text() {
        let ansi_string = color_print::cstr!(r#"<dim>12345</dim>"#);
        let result = parse_ansi_styled_str(ansi_string);
        assert_eq!(
            result,
            vec![("12345", HashSet::from([TextFormattingOverride::Dim])),]
        );
    }

    #[test]
    fn test_strikethrough_text() {
        let ansi_string = color_print::cstr!(r#"<strike>12345</strike>"#);
        let result = parse_ansi_styled_str(ansi_string);
        assert_eq!(
            result,
            vec![(
                "12345",
                HashSet::from([TextFormattingOverride::Strikethrough])
            ),]
        );
    }

    #[test]
    fn test_foreground_color() {
        let ansi_string = color_print::cstr!(r#"<red>12345</red>"#);
        let result = parse_ansi_styled_str(ansi_string);
        assert_eq!(
            result,
            vec![(
                "12345",
                HashSet::from([TextFormattingOverride::Foreground(Color32::from_rgb(
                    222, 56, 43
                ))])
            ),]
        );
    }

    #[test]
    fn test_background_color() {
        let ansi_string = color_print::cstr!(r#"<bg:red>12345</bg:red>"#);
        let result = parse_ansi_styled_str(ansi_string);
        assert_eq!(
            result,
            vec![(
                "12345",
                HashSet::from([TextFormattingOverride::Background(Color32::from_rgb(
                    222, 56, 43
                ))])
            ),]
        );
    }

    #[test]
    fn test_multiple_styles() {
        let ansi_string = color_print::cstr!(r#"<bold><red>12345</red></bold>"#);
        let result = parse_ansi_styled_str(ansi_string);
        assert_eq!(
            result,
            vec![(
                "12345",
                HashSet::from([
                    TextFormattingOverride::Foreground(Color32::from_rgb(222, 56, 43)),
                    TextFormattingOverride::Bold,
                ])
            ),]
        );
    }

    #[test]
    fn non_overlapping_styles() {
        let ansi_string = color_print::cstr!(r#"<bold>12345</bold><red>12345</red>"#);
        let result = parse_ansi_styled_str(ansi_string);
        assert_eq!(
            result,
            vec![
                ("12345", HashSet::from([TextFormattingOverride::Bold])),
                (
                    "12345",
                    HashSet::from([TextFormattingOverride::Foreground(Color32::from_rgb(
                        222, 56, 43
                    ))])
                ),
            ]
        );
    }

    #[test]
    fn overlapping_non_symmetric_styles() {
        let ansi_string = color_print::cstr!(r#"<bold>12345<red>12345</red></bold>"#);
        let result = parse_ansi_styled_str(ansi_string);
        assert_eq!(
            result,
            vec![
                ("12345", HashSet::from([TextFormattingOverride::Bold,])),
                (
                    "12345",
                    HashSet::from([
                        TextFormattingOverride::Bold,
                        TextFormattingOverride::Foreground(Color32::from_rgb(222, 56, 43))
                    ])
                ),
            ]
        );
    }

    #[test]
    fn overlapping_non_symmetric_styles_followed_by_non_styled() {
        let ansi_string = color_print::cstr!(r#"<bold>12345<red>12345</red></bold>end"#);
        let result = parse_ansi_styled_str(ansi_string);
        assert_eq!(
            result,
            vec![
                ("12345", HashSet::from([TextFormattingOverride::Bold,])),
                (
                    "12345",
                    HashSet::from([
                        TextFormattingOverride::Bold,
                        TextFormattingOverride::Foreground(Color32::from_rgb(222, 56, 43))
                    ])
                ),
                ("end", HashSet::from([])),
            ]
        );
    }

    #[test]
    fn test_complex_example() {
        let example = "\0\0\u{1b}[2m2025-01-03T16:58:02.690280Z\u{1b}[0m \u{1b}[31mERROR\u{1b}[0m error: Could not find function: Displaying ScriptValue without world access: String(";

        let result = parse_ansi_styled_str(example);

        assert_eq!(
            result,
            vec![
                ("\0\0", HashSet::from([])),
                (
                    "2025-01-03T16:58:02.690280Z",
                    HashSet::from([TextFormattingOverride::Dim])
                ),
                (" ", HashSet::from([])),
                (
                    "ERROR",
                    HashSet::from([TextFormattingOverride::Foreground(Color32::from_rgb(222, 56, 43))])
                ),
                (" error: Could not find function: Displaying ScriptValue without world access: String(", HashSet::from([])),
            ]
        );
    }

    #[test]
    fn test_complex_example_2() {
        let example = "\u{1b}[2m2025-01-03T19:20:08.083551Z\u{1b}[0m \u{1b}[32m INFO\u{1b}[0m Bye!";

        let result = parse_ansi_styled_str(example);

        assert_eq!(
            result,
            vec![
                (
                    "2025-01-03T19:20:08.083551Z",
                    HashSet::from([TextFormattingOverride::Dim])
                ),
                (" ", HashSet::from([])),
                (
                    " INFO",
                    HashSet::from([TextFormattingOverride::Foreground(Color32::from_rgb(
                        57, 181, 74
                    ))])
                ),
                (" Bye!", HashSet::from([])),
            ]
        );
    }
}
