---
minutes: 5
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Sealed Traits

Traits are normally extensible, meaning that downstream users can implement our
traits for their own types, and then our generic code can handle those
downstream types. Usually this is what we want, but sometimes we want to define
a trait where we control all of the implementations. To do this, we can create a
**sealed trait**.

```rust,editable
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
// Private module, preventing users from accessing the `Sealed` trait.
mod sealed {
    // The trait itself must be public, or we can't use it as
    // part of our public API.
    pub trait Sealed {}

    // Implement `Sealed` for types that should implement our
    // public public.
    impl Sealed for String {}
    impl Sealed for Vec<u8> {}
}

// Use `Sealed` as the supertrait for our public trait.
pub trait APITrait: sealed::Sealed {
    /* methods */
}

impl APITrait for String {}
impl APITrait for Vec<u8> {}
```

<details>

- Motivation: We want trait-driven code in a crate, but we don't want projects
  that depend on this crate to be able to implement a trait.

<!-- TODO: This could use some specific motivating examples. -->

- The mechanism we use to do this is restricting access to a supertrait,
  preventing downstream users from being able to implement that trait for their
  types.

- This is similar to enums, which inherently wrap a limited set of types, but
  applies to traits, which by default are extensible.

</details>
