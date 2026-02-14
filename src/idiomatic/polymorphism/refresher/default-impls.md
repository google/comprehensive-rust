---
minutes: 5
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Default Method Implementations

```rust
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
pub trait CollectLeaves {
    type Leaf;

    // Required Method
    fn collect_leaves_buffered(&self, buf: &mut Vec<Self::Leaf>);

    // Default implementation
    fn collect_leaves(&self) -> Vec<Self::Leaf> {
        let mut buf = vec![];
        self.collect_leaves_buffered(&mut buf);
        buf
    }
}
```

<details>

- Traits often have methods that are implemented for you already, once you
  implement the required methods.

- A trait method has a default implementation if the function body is present.
  This implementation can be written in terms of other methods available, such
  as other methods in the trait or methods of a supertrait.

- Often you'll see methods that provide the broad functionality that is
  necessary to implement (like `Ord`'s `compare`) with default implementations
  for functions that can be implemented in terms of those methods (like `Ord`'s
  `max`/`min`/`clamp`).

- Default methods can be overridden by derive macros, as derive macros produce
  arbitrary ASTs in the implementation.

ref:

- https://doc.rust-lang.org/reference/items/traits.html#r-items.traits.associated-item-decls

</details>
