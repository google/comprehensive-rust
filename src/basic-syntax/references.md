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
