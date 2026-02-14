---
minutes: 5
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Meaningful Doc Comments

```rust,compile_fail
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
/// API for the client // ❌ Lacks detail
pub mod client {}

/// Function from A to B // ❌ Redundant
fn a_to_b(a: A) -> B {...}
 
/// Connects to the database. // ❌ Lacks detail
fn connect() -> Result<(), Error> {...}
```

<details>

- Doc comments are the most common form of documentation developers engage with.

- Good doc comments provide information that the code, names, and types cannot,
  without restating the obvious information.

</details>
