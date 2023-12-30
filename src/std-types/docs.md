---
minutes: 5
---

# Documentation

Rust comes with extensive documentation. For example:

- All of the details about
  [loops](https://doc.rust-lang.org/stable/reference/expressions/loop-expr.html).
- Primitive types like
  [`u8`](https://doc.rust-lang.org/stable/std/primitive.u8.html).
- Standard library types like
  [`Option`](https://doc.rust-lang.org/stable/std/option/enum.Option.html) or
  [`BinaryHeap`](https://doc.rust-lang.org/stable/std/collections/struct.BinaryHeap.html).

In fact, you can document your own code:

```rust,editable
/// Determine whether the first argument is divisible by the second argument.
///
/// If the second argument is zero, the result is false.
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    lhs % rhs == 0
}
```

The contents are treated as Markdown. All published Rust library crates are
automatically documented at [`docs.rs`](https://docs.rs) using the
[rustdoc](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html) tool. It is
idiomatic to document all public items in an API using this pattern.

To document an item from inside the item (such as inside a module), use `//!` or
`/*! .. */`, called "inner doc comments":

```rust,editable
//! This module contains functionality relating to divisibility of integers.
```

<details>

- Show students the generated docs for the `rand` crate at
  <https://docs.rs/rand>.

</details>
