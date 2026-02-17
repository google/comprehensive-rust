<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# safe-mmio

The [`safe-mmio`] crate provides types to wrap registers that can be read or
written safely.

|             | Can't read    | Read has no side-effects | Read has side-effects |
| ----------- | ------------- | ------------------------ | --------------------- |
| Can't write |               | [`ReadPure`]             | [`ReadOnly`]          |
| Can write   | [`WriteOnly`] | [`ReadPureWrite`]        | [`ReadWrite`]         |

<!-- mdbook-xgettext: skip -->

```rust,editable,compile_fail
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
{{#include ../examples/src/pl011.rs:Registers}}
```

- Reading `dr` has a side effect: it pops a byte from the receive FIFO.
- Reading `rsr` (and other registers) has no side-effects. It is a 'pure' read.

<details>

- There are a number of different crates providing safe abstractions around MMIO
  operations; we recommend the `safe-mmio` crate.
- The difference between `ReadPure` or `ReadOnly` (and likewise between
  `ReadPureWrite` and `ReadWrite`) is whether reading a register can have
  side-effects that change the state of the device, e.g., reading the data
  register pops a byte from the receive FIFO. `ReadPure` means that reads have
  no side-effects, they are purely reading data.

</details>

[`safe-mmio`]: https://crates.io/crates/safe-mmio
[`ReadOnly`]: https://docs.rs/safe-mmio/latest/safe_mmio/fields/struct.ReadOnly.html
[`ReadPure`]: https://docs.rs/safe-mmio/latest/safe_mmio/fields/struct.ReadPure.html
[`ReadPureWrite`]: https://docs.rs/safe-mmio/latest/safe_mmio/fields/struct.ReadPureWrite.html
[`ReadWrite`]: https://docs.rs/safe-mmio/latest/safe_mmio/fields/struct.ReadWrite.html
[`WriteOnly`]: https://docs.rs/safe-mmio/latest/safe_mmio/fields/struct.WriteOnly.html
