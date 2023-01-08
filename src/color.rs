use bevy_egui::egui::{Color32, TextFormat};

macro_rules! ok {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => {
                return Err(err);
            }
        }
    };
}

/// Mirror of `clap::Style`
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Style {
    /// names of flags, commands etc
    Header,
    /// Command literal
    Literal,
    /// Basic formatting
    Placeholder,
    /// Used for reporting success
    Good,
    /// Used for reporting warnings
    Warning,
    /// Used for reporting errors
    Error,
    /// Tooltips
    Hint,
}

pub fn style_richtext(style: Option<Style>, mut text: TextFormat) -> TextFormat {
    match style {
        Some(Style::Header) => {
            text.italics = true;
            text.underline = (0.5f32, Color32::GRAY).into()
        }
        Some(Style::Literal) => text.italics = true,
        Some(Style::Placeholder) => {}
        Some(Style::Good) => text.color = Color32::DARK_GREEN,
        Some(Style::Warning) => text.color = Color32::GOLD,
        Some(Style::Error) => {
            text.italics = true;
            text.color = Color32::DARK_RED
        }
        Some(Style::Hint) => text.font_id.size = 10f32,
        None => {}
    }
    text
}

impl Style {
    fn as_usize(&self) -> usize {
        match self {
            Self::Header => 0,
            Self::Literal => 1,
            Self::Placeholder => 2,
            Self::Good => 3,
            Self::Warning => 4,
            Self::Error => 5,
            Self::Hint => 6,
        }
    }
}

/// Mirror of `clap::StyledStr`
#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct StyledStr {
    pieces: Vec<(Option<Style>, String)>,
}

impl From<clap::builder::StyledStr> for StyledStr {
    fn from(value: clap::builder::StyledStr) -> Self {
        // Safety: identical types
        unsafe { std::mem::transmute(value) }
    }
}

impl StyledStr {
    /// Create an empty buffer
    pub const fn new() -> Self {
        Self { pieces: Vec::new() }
    }

    /// Creates buffer with initial message and style
    pub fn new_styled(style: Style, msg: impl Into<String>) -> Self {
        let mut _str = Self::new();
        _str.stylize_(Some(style), msg.into());
        _str
    }

    /// push new header style message
    pub fn header(&mut self, msg: impl Into<String>) -> &mut Self {
        self.stylize_(Some(Style::Header), msg.into());
        self
    }

    /// push new literal style message
    pub fn literal(&mut self, msg: impl Into<String>) -> &mut Self {
        self.stylize_(Some(Style::Literal), msg.into());
        self
    }

    /// push new placeholder style message
    pub fn placeholder(&mut self, msg: impl Into<String>) -> &mut Self {
        self.stylize_(Some(Style::Placeholder), msg.into());
        self
    }

    /// push new error style message
    pub fn error(&mut self, msg: impl Into<String>) -> &mut Self {
        self.stylize_(Some(Style::Error), msg.into());
        self
    }

    /// push new hint style message
    #[allow(dead_code)]
    pub fn hint(&mut self, msg: impl Into<String>) -> &mut Self {
        self.stylize_(Some(Style::Hint), msg.into());
        self
    }

    /// push new message without any styling
    pub fn none(&mut self, msg: impl Into<String>) -> &mut Self {
        self.stylize_(None, msg.into());
        self
    }

    pub(crate) fn stylize(&mut self, style: impl Into<Option<Style>>, msg: impl Into<String>) {
        self.stylize_(style.into(), msg.into());
    }

    #[allow(dead_code)]
    pub(crate) fn trim(&mut self) {
        self.trim_start();
        self.trim_end();
    }

    #[allow(dead_code)]
    pub(crate) fn trim_start(&mut self) {
        if let Some((_, item)) = self.iter_mut().next() {
            *item = item.trim_start().to_owned();
        }
    }

    #[allow(dead_code)]
    pub(crate) fn trim_end(&mut self) {
        if let Some((_, item)) = self.pieces.last_mut() {
            *item = item.trim_end().to_owned();
        }
    }

    #[allow(dead_code)]
    pub(crate) fn indent(&mut self, initial: &str, trailing: &str) {
        if let Some((_, first)) = self.iter_mut().next() {
            first.insert_str(0, initial);
        }
        let mut line_sep = "\n".to_owned();
        line_sep.push_str(trailing);
        for (_, content) in self.iter_mut() {
            *content = content.replace('\n', &line_sep);
        }
    }

    fn stylize_(&mut self, style: Option<Style>, msg: String) {
        if !msg.is_empty() {
            self.pieces.push((style, msg));
        }
    }

    /// Check the string is empty
    pub fn is_empty(&self) -> bool {
        self.pieces.is_empty()
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (Option<Style>, &str)> {
        self.pieces.iter().map(|(s, c)| (*s, c.as_str()))
    }

    pub(crate) fn iter_mut(&mut self) -> impl Iterator<Item = (Option<Style>, &mut String)> {
        self.pieces.iter_mut().map(|(s, c)| (*s, c))
    }

    #[allow(dead_code)]
    pub(crate) fn into_iter(self) -> impl Iterator<Item = (Option<Style>, String)> {
        self.pieces.into_iter()
    }

    /// Extend this string with the contents of an iterator
    pub fn extend(
        &mut self,
        other: impl IntoIterator<Item = (impl Into<Option<Style>>, impl Into<String>)>,
    ) {
        for (style, content) in other {
            self.stylize(style.into(), content.into());
        }
    }
}

impl Default for &'_ StyledStr {
    fn default() -> Self {
        static DEFAULT: StyledStr = StyledStr::new();
        &DEFAULT
    }
}

impl From<std::string::String> for StyledStr {
    fn from(name: std::string::String) -> Self {
        let mut styled = StyledStr::new();
        styled.none(name);
        styled
    }
}

impl From<&'_ std::string::String> for StyledStr {
    fn from(name: &'_ std::string::String) -> Self {
        let mut styled = StyledStr::new();
        styled.none(name);
        styled
    }
}

impl From<&'static str> for StyledStr {
    fn from(name: &'static str) -> Self {
        let mut styled = StyledStr::new();
        styled.none(name);
        styled
    }
}

impl From<&'_ &'static str> for StyledStr {
    fn from(name: &'_ &'static str) -> Self {
        StyledStr::from(*name)
    }
}

impl PartialOrd for StyledStr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for StyledStr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.iter().map(cmp_key).cmp(other.iter().map(cmp_key))
    }
}

fn cmp_key(c: (Option<Style>, &str)) -> (Option<usize>, &str) {
    let style = c.0.map(|s| s.as_usize());
    let content = c.1;
    (style, content)
}

/// Color-unaware printing. Never uses coloring.
impl std::fmt::Display for StyledStr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (_, content) in self.iter() {
            ok!(std::fmt::Display::fmt(content, f));
        }

        Ok(())
    }
}
