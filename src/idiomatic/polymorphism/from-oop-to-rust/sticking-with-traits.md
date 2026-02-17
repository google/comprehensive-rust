---
minutes: 2
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Traits for Polymorphism users can extend

```rust
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
// Crate A

pub trait Trait {
    fn use_trait(&self) {}
}

// Crate B, depends on A

pub struct Data(u8);

impl Trait for Data {}

fn main() {
    let data = Data(7u8);
    data.use_trait();
}
```

<details>

- We've already covered normal traits at length, but compared to enums and
  sealed traits they allow users to extend an API by implementing the behavior
  that API asks of them.

This ability for users to extend is powerful for a number of domains, from
serialization to abstract representations of hardware and type safe linear
algebra.

- If a trait is exposed publicly in a crate, a user depending on that crate can
  implement that trait for types they define.

</details>
