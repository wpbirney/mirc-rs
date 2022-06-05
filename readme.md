# mirc-rs

[![Crates.io](https://img.shields.io/crates/v/mirc)](https://crates.io/crates/mirc/)

A simple mirc color code formatter partially inspired by [yansi](https://github.com/SergioBenitez/yansi)

[docs.rs](https://docs.rs/mirc/latest/mirc/index.html)

## Usage

```rust
use mirc::Paint;

irc.send_privmsg("#channel", Paint::red("red text"));
irc.send_privmsg("#channel", format!("Hello: {}", Paint::blue("nick")));
```

Works on any type that impl's fmt::Display