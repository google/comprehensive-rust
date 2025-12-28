---
minutes: 10
---

# Dyn-compatible traits

```rust
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

  For methods like `clone`, this disqualifies dyn compatibility because the
  output type depends on the concrete type of `self`.

ref:

- https://doc.rust-lang.org/1.91.1/reference/items/traits.html#r-items.traits.dyn-compatible
