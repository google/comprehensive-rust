---
minutes: 10
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Dyn-compatible traits

```rust,editable
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
pub trait Trait {
    // dyn compatible
    fn takes_self(&self);

    // dyn compatible, but you can't use this method when it's dyn
    fn takes_self_and_param<T>(&self, input: &T);

    // no longer dyn compatible
    const ASSOC_CONST: i32;

    // no longer dyn compatible
    fn clone(&self) -> Self;
}
```

<details>

- Not all traits are able to be invoked as trait objects. A trait that can be
  invoked is referred to as a _dyn compatible_ trait.

- This was previously called _object safe traits_ or _object safety_.

- Dynamic dispatch offloads a lot of compile-time type information into runtime
  vtable information.

  If a concept is incompatible with what we can meaningfully store in a vtable,
  either the trait stops being dyn compatible or those methods are excluded from
  being able to be used in a dyn context.

- A trait is dyn-compatible when all its supertraits are dyn-compatible and when
  it has no associated constants/types, and no methods that depend on generics.

- You'll most frequently run into dyn incompatible traits when they have
  associated types/constants or return values of `Self` (i.e. the Clone trait is
  not dyn compatible.)

  This is because the associated data would have to be stored in vtables, taking
  up extra memory.
  <!-- This isn't right, the issue isn't storing the info in the vtable takes memory,
    it's that we fundamentally can't reason about an associated type through a
    vtable. A vtable can hold function pointers, but there's no way to use a
    pointer to point at a type.

    Using an associated type as part of a method's signature means that the
    method has a different signature depending on which type implements the
    trait, and calling a function through a vtable requires that all pointed-to
    functions have exactly the same signature. -->

  For methods like `clone`, this disqualifies dyn compatibility because the
  output type depends on the concrete type of `self`.
  <!-- Likewise, the issue here is that we don't know the `Self` type when going
    through dyn, so we can't call a function that returns Self because the
    different implementations of that function return different types. -->

ref:

- https://doc.rust-lang.org/1.91.1/reference/items/traits.html#r-items.traits.dyn-compatible

</details>
