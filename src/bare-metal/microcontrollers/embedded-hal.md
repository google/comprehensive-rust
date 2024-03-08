# `embedded-hal`

The [`embedded-hal`] crate provides a number of traits covering common
microcontroller peripherals:

- GPIO
- PWM
- Delay timers
- I2C and SPI buses and devices

Similar traits for byte streams (e.g. UARTs), CAN buses and RNGs and broken out
into [`embedded-io`], [`embedded-can`] and [`rand_core`] respectively.

Other crates then implement [drivers] in terms of these traits, e.g. an
accelerometer driver might need an I2C or SPI device instance.

<details>

- The traits cover using the peripherals but not initialising or configuring
  them, as initialisation and configuration is usually highly platform-specific.
- There are implementations for many microcontrollers, as well as other
  platforms such as Linux on Raspberry Pi.
- [`embedded-hal-async`] provides async versions of the traits.
- [`embedded-hal-nb`] provides another approach to non-blocking I/O, based on
  the [`nb`] crate.

</details>

[drivers]: https://github.com/rust-embedded/awesome-embedded-rust#driver-crates
[`embedded-can`]: https://crates.io/crates/embedded-can
[`embedded-hal`]: https://crates.io/crates/embedded-hal
[`embedded-hal-async`]: https://crates.io/crates/embedded-hal-async
[`embedded-hal-nb`]: https://crates.io/crates/embedded-hal-nb
[`embedded-io`]: https://crates.io/crates/embedded-io
[`nb`]: https://crates.io/crates/nb
[`rand_core`]: https://crates.io/crates/rand_core
