# mirc-rs

A simple mirc color code formatter partially inspired by [yansi](https://github.com/SergioBenitez/yansi)

## Usage

```rust
use mirc::Paint;

irc.send_privmsg("#channel", Paint::red("red text"));
irc.send_privmsg("#channel", format!("Hello: {}", Paint::blue("nick")));
```

Works on any type that impl's fmt::Display