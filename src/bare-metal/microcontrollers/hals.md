# HAL crates

[HAL crates](https://github.com/rust-embedded/awesome-embedded-rust#hal-implementation-crates) for
many microcontrollers provide wrappers around various peripherals. These generally implement traits
from [`embedded-hal`](https://crates.io/crates/embedded-hal).

```rust,editable,compile_fail
{{#include examples/src/bin/hal.rs:Example}}
```

<details>

 * `set_low` and `set_high` are methods on the `embedded_hal` `OutputPin` trait.
 * HAL crates exist for many Cortex-M and RISC-V devices, including various STM32, GD32, nRF, NXP,
   MSP430, AVR and PIC microcontrollers.

</details>
