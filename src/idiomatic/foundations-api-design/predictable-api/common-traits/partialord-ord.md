---
minutes: 10
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# PartialOrd and Ord

Partial ordering & Total ordering.

Derivable: ✅

When to implement: Almost always.

```rust
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
// pub trait PartialOrd<Rhs = Self>: PartialEq<Rhs>
// {
//     // Required method
//     fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;
//
//     /* Provided methods omitted */
// }
// pub trait Ord: Eq + PartialOrd {
//     // Required method
//     fn cmp(&self, other: &Self) -> Ordering;
//
//     /* Provided methods omitted */
// }

#[derive(PartialEq, PartialOrd)]
pub struct Partially(f32);

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Totally {
    id: u32,
    name: String,
}
```

<details>
- Comparison-related methods. If a type implements `PartialOrd`/`Ord` then you can use comparison operators (`<`, `<=`, `>`, `>=`) with that type.

`Ord` gives access to `min`, `max`, and `clamp` methods.

- When derived, compares things in the order they are defined.

  For enums this means each variant is considered "greater than" the last as
  they are written.

  For structs this means fields are compared as they are written, so `id` fields
  are compared before `name` fields in `Totally`.

- Prerequisites: `PartialEq` for `PartialOrd`, `Eq` for `Ord`.

  To implement `Ord`, a type must also implement `PartialEq`, `Eq`, and
  `PartialOrd`.

- Like with `PartialEq` and `Eq`, a type cannot implement `Ord` without
  implementing `PartialOrd`.

  Like those equality traits, `PartialOrd` exists to separate types with
  non-total ordering (particularly floating-point numbers) from types with total
  ordering.

- Used for sorting/searching algorithms and maintaining the ordering of
  `BTreeMap`/`BTreeSet` style data types.

</details>
