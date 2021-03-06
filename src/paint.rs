use crate::Color;

use std::fmt;

enum Style {
    Default,
    Bold,
    Italic,
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Style::Default => Ok(()),
            Style::Bold => write!(f, "\x02"),
            Style::Italic => write!(f, "\x1D"),
        }
    }
}

/// Simple structure for handling most basic mirc color painting needs
pub struct Paint<T> {
    fg: Color,
    bg: Color,
    style: Style,
    content: T,
}

impl<T> fmt::Display for Paint<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.bg {
            Color::Unset => write!(
                f,
                "{}\x03{:02}{}\x03{}",
                self.style, self.fg, self.content, self.style
            ),
            _ => write!(
                f,
                "{}\x03{:02},{:02}{}\x03{}",
                self.style, self.fg, self.bg, self.content, self.style
            ),
        }
    }
}

macro_rules! constructors_for {
    ($T:ty, $($name:ident: $color:ident),*) => ($(
        #[inline]
        pub fn $name(content: $T) -> Paint<$T> {
            Paint::new(Color::$color, Color::Unset, content)
        }
    )*);
}

impl<T> Paint<T> {
    pub fn new(fg: Color, bg: Color, content: T) -> Paint<T> {
        Paint {
            fg,
            bg,
            style: Style::Default,
            content,
        }
    }

    constructors_for!(
        T,
        white: White,
        black: Black,
        blue: Blue,
        green: Green,
        red: Red,
        brown: Brown,
        purple: Purple,
        orange: Orange,
        yellow: Yellow,
        lightgreen: LightGreen,
        cyan: Cyan,
        lightcyan: LightCyan,
        lightblue: LightBlue,
        pink: Pink,
        grey: Grey,
        lightgrey: LightGrey
    );

    /// Sets background to specified [Color]
    pub fn bg(mut self, bg: Color) -> Paint<T> {
        self.bg = bg;
        self
    }

    /// Adds bold styling to the content
    ///
    /// ```rust
    /// use mirc::Paint;
    ///
    /// irc.send_privmsg("#channel", format!("{}", Paint::green("hello world").bold()));
    /// ```
    pub fn bold(mut self) -> Paint<T> {
        self.style = Style::Bold;
        self
    }

    /// Adds italic styling to the content
    ///
    /// ```rust
    /// use mirc::Paint;
    ///
    /// irc.send_privmsg("#channel", format!("{}", Paint::green("hello world").italics()));
    /// ```
    pub fn italic(mut self) -> Paint<T> {
        self.style = Style::Italic;
        self
    }
}
