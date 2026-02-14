<!--
Copyright 2023 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Peripheral Access Crates

[`svd2rust`](https://crates.io/crates/svd2rust) generates mostly-safe Rust
wrappers for memory-mapped peripherals from
[CMSIS-SVD](https://www.keil.com/pack/doc/CMSIS/SVD/html/index.html) files.

```rust,editable,compile_fail
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
{{#include examples/src/bin/pac.rs:Example}}
```

<details>

- SVD (System View Description) files are XML files typically provided by
  silicon vendors that describe the memory map of the device.
  - They are organized by peripheral, register, field and value, with names,
    descriptions, addresses and so on.
  - SVD files are frequently buggy and incomplete, so there are various projects
    that patch the mistakes, add missing details, and publish the generated
    crates.
- `cortex-m-rt` provides the vector table, among other things.
- If you `cargo install cargo-binutils` then you can run
  `cargo objdump --bin pac -- -d --no-show-raw-insn` to see the resulting
  binary.

Run the example with:

```sh
cargo embed --bin pac
```

</details>
