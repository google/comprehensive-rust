---
minutes: 5
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Supertraits / Trait Dependencies

Traits can be extended by new traits.

```rust
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
pub trait DeviceId {
    /* trait for device ID types */
}

pub trait GraphicsDevice: DeviceId {
    /* Graphics device specifics */
}

// From stdlib

pub trait Ord: Eq + PartialOrd {
    /* methods for Ord */
}
```

<details>

- When authoring a trait, you can specify traits that a type must also. These
  are called _Supertraits_.

  For the example above, any type that implements `GraphicsDevice` must also
  implement `DeviceId`.

- These hierarchies of traits let us design systems around the behavior of
  complex real-world taxonomies (like machine hardware, operating system
  specifics).

- This is distinct from object inheritance! But it looks similar.

  - Object inheritance allows for overrides and brings in the behavior of the
    inherited types by default.

  - A trait having a supertrait doesn't mean that trait can override method
    implementations as default implementations.

ref:

- https://doc.rust-lang.org/reference/items/traits.html?highlight=supertrait#r-items.traits.supertraits

</details>
