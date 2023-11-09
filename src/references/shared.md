---
minutes: 10
existing course material:
- basic-syntax/references.md
- basic-syntax/references-dangling.md
---

<!-- NOTES:
First-pass introduction to references, without owernship, borrow checking, etc. Very informal coverage of lifetimes.
-->
# Shared References

# References

Like C++, Rust has references:

<!-- mdbook-xgettext: skip -->
```rust,editable
fn main() {
    let mut x: i32 = 10;
    let ref_x: &mut i32 = &mut x;
    *ref_x = 20;
    println!("x: {x}");
}
```

Some notes:

* We must dereference `ref_x` when assigning to it, similar to C and C++ pointers.
* Rust will auto-dereference in some cases, in particular when invoking
  methods (try `ref_x.count_ones()`).
* References that are declared as `mut` can be bound to different values over their lifetime.

<details>

Key points:

* Be sure to note the difference between `let mut ref_x: &i32` and `let ref_x:
  &mut i32`. The first one represents a mutable reference which can be bound to
  different values, while the second represents a reference to a mutable value.

</details>
# Dangling References

Rust will statically forbid dangling references:

<!-- mdbook-xgettext: skip -->
```rust,editable,compile_fail
fn main() {
    let ref_x: &i32;
    {
        let x: i32 = 10;
        ref_x = &x;
    }
    println!("ref_x: {ref_x}");
}
```

* A reference is said to "borrow" the value it refers to.
* Rust is tracking the lifetimes of all references to ensure they live long
  enough.
* We will talk more about borrowing when we get to ownership.
