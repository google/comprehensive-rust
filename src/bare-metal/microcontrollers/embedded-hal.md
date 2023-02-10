# `embedded-hal`

The [`embedded-hal`](https://crates.io/crates/embedded-hal) crate provides a number of traits
covering common microcontroller peripherals.

 * GPIO
 * ADC
 * I2C, SPI, UART, CAN
 * RNG
 * Timers
 * Watchdogs

Other crates then implement
[drivers](https://github.com/rust-embedded/awesome-embedded-rust#driver-crates) in terms of these
traits, e.g. an accelerometer driver might need an I2C or SPI bus implementation.

<details>

 * There are implementations for many microcontrollers, as well as other platforms such as Linux on
Raspberry Pi.
 * There is work in progress on an `async` version of `embedded-hal`, but it isn't stable yet.

</details>
