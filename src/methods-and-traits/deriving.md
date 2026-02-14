---
minutes: 3
---

<!--
Copyright 2023 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Deriving

Supported traits can be automatically implemented for your custom types, as
follows:

```rust,editable
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
#[derive(Debug, Clone, Default)]
struct Player {
    name: String,
    strength: u8,
    hit_points: u8,
}

fn main() {
    let p1 = Player::default(); // Default trait adds `default` constructor.
    let mut p2 = p1.clone(); // Clone trait adds `clone` method.
    p2.name = String::from("EldurScrollz");
    // Debug trait adds support for printing with `{:?}`.
    println!("{p1:?} vs. {p2:?}");
}
```

<details>

- Derivation is implemented with macros, and many crates provide useful derive
  macros to add useful functionality. For example, `serde` can derive
  serialization support for a struct using `#[derive(Serialize)]`.

- Derivation is usually provided for traits that have a common boilerplate
  implementation that is correct for most cases. For example, demonstrate how a
  manual `Clone` impl can be repetitive compared to deriving the trait:

  ```rust,ignore
  # // Copyright 2023 Google LLC
  # // SPDX-License-Identifier: Apache-2.0
  #
  impl Clone for Player {
      fn clone(&self) -> Self {
          Player {
              name: self.name.clone(),
              strength: self.strength.clone(),
              hit_points: self.hit_points.clone(),
          }
      }
  }
  ```

  Not all of the `.clone()`s in the above are necessary in this case, but this
  demonstrates the generally boilerplate-y pattern that manual impls would
  follow, which should help make the use of `derive` clear to students.

</details>
