---
minutes: 3
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Copying memory - Introduction

```rust,ignore
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
/// Reads bytes from `source` and writes them to `dest`
pub fn copy(dest: &mut [u8], source: &[u8]) { ... }
```

<details>

“Here is our initial function prototype.”

“`copy` accepts two slices as arguments. `dest` (destination) is mutable,
whereas `source` is not.”

“Let's see the shapes of sound Rust code.”

</details>
