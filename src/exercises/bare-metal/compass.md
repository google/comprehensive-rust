# Compass

We will read the direction from an I2C compass, and log the readings to a serial
port. If you have time, try displaying it on the LEDs somehow too, or use the
buttons somehow.

Hints:

- Check the documentation for the
  [`lsm303agr`](https://docs.rs/lsm303agr/latest/lsm303agr/) and
  [`microbit-v2`](https://docs.rs/microbit-v2/latest/microbit/) crates, as well
  as the [micro:bit hardware](https://tech.microbit.org/hardware/).
- The LSM303AGR Inertial Measurement Unit is connected to the internal I2C bus.
- TWI is another name for I2C, so the I2C master peripheral is called TWIM.
- The LSM303AGR driver needs something implementing the
  `embedded_hal::blocking::i2c::WriteRead` trait. The
  [`microbit::hal::Twim`](https://docs.rs/microbit-v2/latest/microbit/hal/struct.Twim.html)
  struct implements this.
- You have a
  [`microbit::Board`](https://docs.rs/microbit-v2/latest/microbit/struct.Board.html)
  struct with fields for the various pins and peripherals.
- You can also look at the
  [nRF52833 datasheet](https://infocenter.nordicsemi.com/pdf/nRF52833_PS_v1.5.pdf)
  if you want, but it shouldn't be necessary for this exercise.

Download the [exercise template](../../comprehensive-rust-exercises.zip) and
look in the `compass` directory for the following files.

_src/main.rs_:

<!-- File src/main.rs -->

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

_Cargo.toml_ (you shouldn't need to change this):

<!-- File Cargo.toml -->
<!-- mdbook-xgettext: skip -->

```toml
{{#include compass/Cargo.toml}}
```

_Embed.toml_ (you shouldn't need to change this):

<!-- File Embed.toml -->
<!-- mdbook-xgettext: skip -->

```toml
{{#include compass/Embed.toml}}
```

_.cargo/config.toml_ (you shouldn't need to change this):

<!-- File .cargo/config.toml -->
<!-- mdbook-xgettext: skip -->

```toml
{{#include compass/.cargo/config.toml}}
```

See the serial output on Linux with:

<!-- mdbook-xgettext: skip -->

```sh
picocom --baud 115200 --imap lfcrlf /dev/ttyACM0
```

Or on Mac OS something like (the device name may be slightly different):

<!-- mdbook-xgettext: skip -->

```sh
picocom --baud 115200 --imap lfcrlf /dev/tty.usbmodem14502
```

Use Ctrl+A Ctrl+Q to quit picocom.
