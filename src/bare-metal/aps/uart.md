# Let's write a UART driver

The QEMU 'virt' machine has a [PL011][1] UART, so let's write a driver for that.

```rust,editable
{{#include examples/src/pl011_minimal.rs:Example}}
```

<details>

- Note that `Uart::new` is unsafe while the other methods are safe. This is
  because as long as the caller of `Uart::new` guarantees that its safety
  requirements are met (i.e. that there is only ever one instance of the driver
  for a given UART, and nothing else aliasing its address space), then it is
  always safe to call `write_byte` later because we can assume the necessary
  preconditions.
- We could have done it the other way around (making `new` safe but `write_byte`
  unsafe), but that would be much less convenient to use as every place that
  calls `write_byte` would need to reason about the safety
- This is a common pattern for writing safe wrappers of unsafe code: moving the
  burden of proof for soundness from a large number of places to a smaller
  number of places.

</details>

[1]: https://developer.arm.com/documentation/ddi0183/g
