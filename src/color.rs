use std::fmt;

pub enum Color {
    White,
    Black,
    Blue,
    Green,
    Red,
    Brown,
    Purple,
    Orange,
    Yellow,
    LightGreen,
    Cyan,
    LightCyan,
    LightBlue,
    Pink,
    Grey,
    LightGrey,
    Raw(i32),
    Unset,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::White => write!(f, "00"),
            Color::Black => write!(f, "01"),
            Color::Blue => write!(f, "02"),
            Color::Green => write!(f, "03"),
            Color::Red => write!(f, "04"),
            Color::Brown => write!(f, "05"),
            Color::Purple => write!(f, "06"),
            Color::Orange => write!(f, "07"),
            Color::Yellow => write!(f, "08"),
            Color::LightGreen => write!(f, "09"),
            Color::Cyan => write!(f, "10"),
            Color::LightCyan => write!(f, "11"),
            Color::LightBlue => write!(f, "12"),
            Color::Pink => write!(f, "13"),
            Color::Grey => write!(f, "14"),
            Color::LightGrey => write!(f, "15"),
            Color::Raw(c) => write!(f, "{:02}", c),
            Color::Unset => Ok(()),
        }
    }
}
