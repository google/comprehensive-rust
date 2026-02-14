<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Using It

Let's write a small program using our driver to write to the serial console, and
echo incoming bytes.

```rust,editable,compile_fail
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
{{#include ../examples/src/main_safemmio.rs:main}}
```

<details>

- Run the example in QEMU with `make qemu_safemmio` under
  `src/bare-metal/aps/examples`.

</details>
