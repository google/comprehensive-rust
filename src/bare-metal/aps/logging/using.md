<!--
Copyright 2023 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Using it

We need to initialise the logger before we use it.

```rust,editable,compile_fail
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
{{#include ../examples/src/main_logger.rs:main}}
```

<details>

- Note that our panic handler can now log details of panics.
- Run the example in QEMU with `make qemu_logger` under
  `src/bare-metal/aps/examples`.

</details>
