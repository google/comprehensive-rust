# Multiple registers

We can use a struct to represent the memory layout of the UART's registers,
using types from the `safe-mmio` crate to wrap ones which can be read or written
safely.

<!-- mdbook-xgettext: skip -->

```rust,editable,compile_fail
{{#include ../examples/src/pl011.rs:Registers}}
```

<details>

- [`#[repr(C)]`](https://doc.rust-lang.org/reference/type-layout.html#the-c-representation)
  tells the compiler to lay the struct fields out in order, following the same
  rules as C. This is necessary for our struct to have a predictable layout, as
  default Rust representation allows the compiler to (among other things)
  reorder fields however it sees fit.
- There are a number of different crates providing safe abstractions around MMIO
  operations; we recommend the `safe-mmio` crate.
- The difference between `ReadPure` or `ReadOnly` (and likewise between
  `ReadPureWrite` and `ReadWrite`) is whether reading a register can have
  side-effects which change the state of the device. E.g. reading the data
  register pops a byte from the receive FIFO. `ReadPure` means that reads have
  no side-effects, they are purely reading data.

</details>
