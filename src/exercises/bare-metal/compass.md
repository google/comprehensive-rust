# Compass

We will read the direction from an I2C compass, and log the readings to a serial port.

Hint: check the documentation for the [`lsm303agr`](https://docs.rs/lsm303agr/latest/lsm303agr/) and
[`microbit-v2`](https://docs.rs/microbit-v2/latest/microbit/) crates, as well as the
[micro:bit hardware](https://tech.microbit.org/hardware/). The LSM303AGR Inertial Measurement Unit
is connected to the internal I2C bus. TWI is another name for I2C, so the I2C master peripheral is
called TWIM. You can also look at the
[nRF52833 datasheet](https://infocenter.nordicsemi.com/pdf/nRF52833_PS_v1.5.pdf) if you want, but it
shouldn't be necessary for this exercise.

If you have time, try displaying it on the LEDs somehow too, or use the buttons somehow.

`src/main.rs`:
```rust,compile_fail
{{#include compass/src/main.rs:top}}
use microbit::{hal::uarte::{Baudrate, Parity, Uarte}, Board};

{{#include compass/src/main.rs:main}}
    // TODO

{{#include compass/src/main.rs:loop}}
        // TODO
    }
}
```

`Cargo.toml` (you shouldn't need to change this):
```toml
{{#include compass/Cargo.toml}}
```

`Embed.toml` (you shouldn't need to change this):
```toml
{{#include compass/Embed.toml}}
```

`.cargo/config.toml` (you shouldn't need to change this):
```toml
{{#include compass/.cargo/config.toml}}
```

See the serial output with

```sh
picocom --baud 115200 --imap lfcrlf /dev/ttyACM0
```

Use Ctrl+A Ctrl+Q to quit picocom.
