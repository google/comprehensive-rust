---
minutes: 10
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Blanket Trait Implementations

We can use a generic `impl` block to implement a trait for multiple types at
once, generally based on the behavior provided by another trait. These are
referred to as **blanket implementations**.

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

- `impl` blocks can be generic, which allow us to apply an implementation to
  multiple types at once. This is commonly used when applying an `impl` block to
  a generic type, but can also be used to implement a trait for multiple types
  at once.

- When an `impl` block applies to multiple types, we refer this as a "blanket
  impl".

- In the example above we have a blanket implementation for all types that
  implement `Display`.

- Blanket impls can restrict how traits are implemented. Rust prevents a type
  from implementing the same trait twice, so a blanket impl prevents the trait
  from being implemented directly on a covered type.

  - Demonstrate this by adding an implementation for `String` and show the
    resulting error.

  - Avoid a blanket impl if users are likely to want to customize the behavior
    of the implementation beyond what the blanket impl provides.

ref:

- https://doc.rust-lang.org/reference/glossary.html#blanket-implementation

</details>
