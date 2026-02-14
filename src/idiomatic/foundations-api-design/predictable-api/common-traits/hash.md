---
minutes: 2
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Hash

Performing a hash on a type.

Derivable: ✅

When to implement: Almost always.

```rust
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
// pub trait Hash {
//     // Required method
//     fn hash<H>(&self, state: &mut H)
//        where H: Hasher;
//
//     // Provided method
//     fn hash_slice<H>(data: &[Self], state: &mut H)
//        where H: Hasher,
//              Self: Sized { ... }
// }

#[derive(Hash)]
pub struct User {
    id: u32,
    name: String,
    friends: Vec<u32>,
}
```

<details>
- Allows a type to be used in hash algorithms.

- Most commonly used with data structures like `HashMap`.

</details>
