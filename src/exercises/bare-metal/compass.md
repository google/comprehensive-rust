# Compass

We will read the temperature from an I2C compass, and log the readings to a serial port.

Hint: check the documentation for the [`lsm303agr`](https://docs.rs/lsm303agr/latest/lsm303agr/) and
[`microbit-v2`](https://docs.rs/microbit-v2/latest/microbit/) crates.

If you have time, try displaying it on the LEDs somehow too, or use the buttons somehow.

```rust,compile_fail
{{#include compass/src/main.rs:top}}
use microbit::{hal::uarte::{Baudrate, Parity, Uarte}, Board};

{{#include compass/src/main.rs:main}}

{{#include compass/src/main.rs:loop}}
    }
}
```

See the serial output with

```sh
picocom --baud 115200 --imap lfcrlf /dev/ttyACM0
```

Use Ctrl+A Ctrl+Q to quit picocom.
