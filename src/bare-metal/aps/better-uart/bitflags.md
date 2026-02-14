<!--
Copyright 2023 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Bitflags

The [`bitflags`](https://crates.io/crates/bitflags) crate is useful for working
with bitflags.

```rust,editable,compile_fail
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
{{#include ../examples/src/pl011_struct.rs:Flags}}
```

<details>

- The `bitflags!` macro creates a newtype something like `struct Flags(u16)`,
  along with a bunch of method implementations to get and set flags.

</details>
