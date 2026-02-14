<!--
Copyright 2023 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Multiple registers

We can use a struct to represent the memory layout of the UART's registers.

<!-- mdbook-xgettext: skip -->

```rust,editable,compile_fail
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
{{#include ../examples/src/pl011_struct.rs:Registers}}
```

<details>

- [`#[repr(C)]`](https://doc.rust-lang.org/reference/type-layout.html#the-c-representation)
  tells the compiler to lay the struct fields out in order, following the same
  rules as C. This is necessary for our struct to have a predictable layout, as
  default Rust representation allows the compiler to (among other things)
  reorder fields however it sees fit.

</details>
