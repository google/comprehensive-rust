# References

Like C++, Rust has references:

```rust,editable
fn main() {
    let mut x: i32 = 10;
    let ref_x: &mut i32 = &mut x;
    *ref_x = 20;
    println!("x: {x}");
}
```

Some differences from C++:

* We must dereference `ref_x` when assigning to it, similar to C pointers,
* Rust will auto-dereference in some cases, in particular when invoking
  methods (try `count_ones`).
