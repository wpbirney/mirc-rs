use std::fmt;

pub enum ColorCode {
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
    Unset,
}

impl fmt::Display for ColorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColorCode::White => write!(f, "00"),
            ColorCode::Black => write!(f, "01"),
            ColorCode::Blue => write!(f, "02"),
            ColorCode::Green => write!(f, "03"),
            ColorCode::Red => write!(f, "04"),
            ColorCode::Brown => write!(f, "05"),
            ColorCode::Purple => write!(f, "06"),
            ColorCode::Orange => write!(f, "07"),
            ColorCode::Yellow => write!(f, "08"),
            ColorCode::LightGreen => write!(f, "09"),
            ColorCode::Cyan => write!(f, "10"),
            ColorCode::LightCyan => write!(f, "11"),
            ColorCode::LightBlue => write!(f, "12"),
            ColorCode::Pink => write!(f, "13"),
            ColorCode::Grey => write!(f, "14"),
            ColorCode::LightGrey => write!(f, "15"),
            ColorCode::Unset => Ok(()),
        }
    }
}

pub struct Color<T> {
    fg: ColorCode,
    bg: ColorCode,
    content: T,
}

impl<T> fmt::Display for Color<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x03{:02}{}\x03", self.fg, self.content)
    }
}

macro_rules! constructors_for {
    ($T:ty, $($name:ident: $color:ident),*) => ($(
        #[inline]
        pub fn $name(content: $T) -> Color<$T> {
            Color::new(ColorCode::$color, ColorCode::Unset, content)
        }
    )*);
}

impl<T> Color<T> {
    pub fn new(fg: ColorCode, bg: ColorCode, content: T) -> Color<T> {
        Color {
            fg,
            bg,
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
}
