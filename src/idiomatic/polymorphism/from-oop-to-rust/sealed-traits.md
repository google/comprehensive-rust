---
minutes: 5
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Sealed traits for Polymorphism users cannot extend

```rust
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
// crate can access the "sealed" module and its trait, but projects that
// depend on it cannot.
mod sealed {
    pub trait Sealed {}
    impl Sealed for String {}
    impl Sealed for Vec<u8> {}
    //...
}

pub trait APITrait: sealed::Sealed {
    /* methods */
}
impl APITrait for String {}
impl APITrait for Vec<u8> {}
```

<details>

- Motivation: We want trait-driven code in a crate, but we don't want projects
  that depend on this crate to be able to implement a trait.

Why?

The trait could be considered unstable for downstream-implementations at this
point in time.

Alternatively: Domain is high-risk for naive implementations of a trait (such as
cryptography).

- The mechanism we use to do this is restricting access to a supertrait,
  preventing downstream users from being able to implement that trait for their
  types.

- Why not just use enums?

  - Enums expose implementation details – "this works for these types".

  - Users need to use variant constructors of an enum to use the API.

  - Users can use the enum as a type in their own code, and when the enum
    changes users need to update their code to match those changes.

  - Enums require branching on variants, whereas sealed traits lets the compile
    specify monomorphized functions for each type.

</details>
