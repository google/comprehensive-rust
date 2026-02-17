<!--
Copyright 2023 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Logging

It would be nice to be able to use the logging macros from the [`log`][1] crate.
We can do this by implementing the `Log` trait.

```rust,editable,compile_fail
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
{{#include examples/src/logger.rs:main}}
```

<details>

- The first unwrap in `log` will succeed because we initialize `LOGGER` before
  calling `set_logger`. The second will succeed because `Uart::write_str` always
  returns `Ok`.

</details>

[1]: https://crates.io/crates/log
