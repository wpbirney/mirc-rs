//! A simple mirc color code formatter partially inspired by yansi
//!
//! # Usage
//!
//! ```rust
//! use mirc::Paint;
//!
//! irc.send_privmsg("#channel", Paint::red("red text"));
//! irc.send_privmsg("#channel", format!("Hello: {}", Paint::blue("nick")));
//! ```

mod color;
mod paint;
mod rainbow;

pub use color::Color;
pub use paint::Paint;
pub use rainbow::Rainbow;
