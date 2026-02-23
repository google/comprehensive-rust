---
minutes: 10
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Blanket Trait Implementations

When a trait is local, we can implement it for as many types as we like. How far
can we take this?

```rust,editable
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
pub trait PrettyPrint {
    fn pretty_print(&self);
}

// A blanket implementation! If something implements Display, it implements
// PrettyPrint.
impl<T> PrettyPrint for T
where
    T: std::fmt::Display,
{
    fn pretty_print(&self) {
        println!("{self}")
    }
}
```

<details>

- The subject of a trait implementation at the definition site of a trait can be
  anything, including `T` with no bounds.

  We can't do anything with a `T` we don't know nothing about, so this is
  uncommon.

- Conditional blanket implementations are much more useful and you are more
  likely to see and author them.

  These implementations will have a bound on the trait, like
  `impl <T: Display> ToString for T {...}`

  In the example above we have a blanket implementation for all types that
  implement Display, the implementation has one piece of information available
  to it from the trait bounds: it implements `Display::fmt`.

  This is enough to write an implementation for pretty printing to console.

- Do be careful with these kinds of implementations, as it may end up preventing
  users downstream from implementing a more meaningful.

  The above isn't written for `Debug` as that would mean almost all types end up
  implementing `PrettyPrint`, and `Debug` is not semantically similar to
  `Display`: It's meant for debug output instead of something more
  human-readable.

ref:

- https://doc.rust-lang.org/reference/glossary.html#blanket-implementation

</details>
