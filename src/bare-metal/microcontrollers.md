<!--
Copyright 2023 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Microcontrollers

The `cortex_m_rt` crate provides (among other things) a reset handler for Cortex
M microcontrollers.

<!-- mdbook-xgettext: skip -->

```rust,editable,compile_fail
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
{{#include microcontrollers/examples/src/bin/minimal.rs:Example}}
```

Next we'll look at how to access peripherals, with increasing levels of
abstraction.

<details>

- The `cortex_m_rt::entry` macro requires that the function have type
  `fn() -> !`, because returning to the reset handler doesn't make sense.
- Run the example with `cargo embed --bin minimal`

</details>
