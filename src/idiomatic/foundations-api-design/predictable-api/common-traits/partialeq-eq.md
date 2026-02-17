---
minutes: 10
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

PartialEq and Eq

Partial equality & Total equality.

Derivable: ✅

When to implement: Almost always.

```rust
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
// pub trait PartialEq<Rhs = Self>
//{
//    // Required method
//     fn eq(&self, other: &Rhs) -> bool;
// 
//     // Provided method
//     fn ne(&self, other: &Rhs) -> bool { ... }
// }
//
// pub trait Eq: PartialEq { }

#[derive(PartialEq, Eq)]
pub struct User { name: String, favorite_number: i32 }

let alice = User { name: "alice".to_string(), favorite_number: 1_000_042 };
let bob = User { name: "bob".to_string(), favorite_number: 42 };

dbg!(alice == alice);
dbg!(alice == bob);
```

<details>
- Equality-related methods. If a type implements `PartialEq`/`Eq` then you can use the `==` operator with that type.

- A type can't implement `Eq` without implementing `PartialEq`.

- Reminder: Partial means "there are invalid members of this set for this
  function."

  This doesn't mean that equality will panic, or that it returns a result, just
  that there may be values that may not behave as you expect equality to behave.

  For example, with floating point values `NaN` is an outlier: `NaN == NaN` is
  false, despite bitwise equality.

  `PartialEq` exists to separate types like f32/f64 from types with Total
  Equality.

- You can implement `PartialEq` between different types, but this is mostly
  useful for reference/smart pointer types.

</details>
