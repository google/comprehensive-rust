# Raw MMIO

Most microcontrollers access peripherals via memory-mapped IO. Let's try turning
on an LED on our micro:bit:

```rust,editable,compile_fail
{{#include examples/src/bin/mmio.rs:Example}}
```

<details>

- GPIO 0 pin 21 is connected to the first column of the LED matrix, and pin 28
  to the first row.

Run the example with:

```sh
cargo embed --bin mmio
```

</details>
