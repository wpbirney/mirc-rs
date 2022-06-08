use std::fmt;

use crate::color::Color;

/// Structure for handling painting items as rainbows!
pub struct Rainbow<'r> {
    colors: Vec<Color>,
    content: &'r str,
}

impl fmt::Display for Rainbow<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut idx = 0;

        for c in self.content.chars() {
            write!(f, "\x03{:02}{}", self.colors[idx], c)?;
            idx += 1;
            if idx == self.colors.len() {
                idx = 0;
            }
        }

        write!(f, "\x03")
    }
}

impl<'r> Rainbow<'r> {
    /// Paint a &str using a custom set of colors in a rainbow
    /// ```rust
    /// use mirc::{Rainbow, Color};
    ///
    /// let msg = Rainbow::new("hello world!", vec![Color::Green, Color::Red, Color::Blue]);
    ///
    /// irc.send_privmsg("#channel", format!("{}", msg));
    /// ```
    pub fn new(content: &'r str, colors: Vec<Color>) -> Rainbow {
        Rainbow { colors, content }
    }

    /// Paints a &str using a default set of rainbow colors
    /// ```rust
    /// use mirc::Rainbow;
    ///
    /// irc.send_privmsg("#channel", format!("{}", Rainbow::default("HELLO WORLD!")));
    /// ```
    pub fn default(content: &'r str) -> Rainbow {
        Rainbow {
            colors: vec![
                Color::Red,
                Color::Orange,
                Color::Yellow,
                Color::Green,
                Color::Blue,
                Color::Purple,
                Color::Pink,
            ],
            content: content,
        }
    }
}
