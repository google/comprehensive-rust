# Let's write a UART driver

The QEMU 'virt' machine has a [PL011][1] UART, so let's write a driver for that.

```rust,editable,compile_fail
{{#include examples/src/pl011_minimal.rs:Example}}
```

- [1]: https://developer.arm.com/documentation/ddi0183/g
