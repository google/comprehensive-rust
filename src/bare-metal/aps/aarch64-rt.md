<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# aarch64-rt

The `aarch64-rt` crate provides the assembly entry point and exception vector
that we implemented before. We just need to mark our main function with the
`entry!` macro.

It also provides the `initial_pagetable!` macro to let us define an initial
static pagetable in Rust, rather than in assembly code like we did before.

We can also use the UART driver from the `arm-pl011-uart` crate rather than
writing our own.

```rust,editable,compile_fail
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
{{#include examples/src/main_rt.rs:main}}
```

<details>

- Run the example in QEMU with `make qemu_rt` under
  `src/bare-metal/aps/examples`.

</details>
