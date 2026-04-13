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

```rust,editable
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
pub struct User {
    id: u32,
    name: String,
}

fn main() {
    let user = User { id: 1, name: "Alice".into() };
    let mut map = HashMap::new();
    map.insert(user, "value");
}
```

<details>

- Allows a type to be used in hash algorithms, most commonly used with data
  structures like `HashMap`.

- Makes it very easy for us to use custom types as the keys in a `HashMap`!

- `Hash` doesn't define any of the hashing logic itself, instead it just feeds
  the type's data into a `Hasher`. This allows us to use different hash
  algorithms without changing a type's `Hash` impl.

</details>
