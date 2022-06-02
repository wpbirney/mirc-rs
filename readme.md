# mirc-rs

A simple mirc color code formatter partially inspired by [yansi](https://github.com/SergioBenitez/yansi)

## Usage

```rust
use mirc::Color;

irc.send_privmsg("#channel", Color::red("red text"));
irc.send_privmsg("#channel", format!("Hello: {}", Color::blue("nick")));
```

Works on any type that impl's fmt::Display