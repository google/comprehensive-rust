# Raw MMIO

Most microcontrollers access peripherals via memory-mapped IO. Let's try turning on an LED on our
micro:bit:

```rust,editable,compile_fail
{{#include examples/src/bin/mmio.rs:Example}}
```

<details>

Run the example with:

```sh
cargo embed --bin mmio
```

</details>
