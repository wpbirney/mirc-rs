use std::fmt;

use crate::color::Color;

pub struct Rainbow<'r>  {
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
    pub fn new(content: &'r str, colors: Vec<Color>) -> Rainbow {
        Rainbow { colors, content }
    }

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
