---
minutes: 5
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Inheritance from Rust's Perspective

```rust
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
// Data
pub struct Data {
    id: usize,
    name: String,
}

// Concrete behavior
impl Data {
    fn new(id: usize, name: impl Into<String>) -> Self {
        Self { id, name: name.into() }
    }
}

// Abstract behavior
trait Named {
    fn name(&self) -> &str;
}

// Instanced behavior
impl Named for Data {
    fn name(&self) -> &str {
        &self.name
    }
}
```

<details>

- From Rust's perspective, one where Inheritance was never there, introducing
  inheritance would look like muddying the water between types and traits.

- A type is a concrete piece of data and its associated behavior.

  A trait is abstract behavior that must be implemented by a type.

  A class is a combination of data, behavior, and overrides to that behavior.

- Coming from Rust, an inheritable class looks like a type that is also a trait.

- This is not an upside, as we can no longer reason about concrete types.

- Without being able to separate the two, it becomes difficult to reason about
  generic behavior vs concrete specifics, because in OOP these two concepts are
  tied up in each other.

- The convenience of flat field access and DRY in type definitions is not worth
  the loss in specificity between writing code that delineates between behavior
  and data.

</details>
