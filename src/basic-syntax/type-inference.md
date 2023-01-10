# Type Inference

Rust will look at how the variable is _used_ to determine the type:

```rust,editable
fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}

fn main() {
    let x = 10;
    let y = 20;

    takes_u32(x);
    takes_i8(y);
    // takes_u32(y);
}
```

<details>

This slide demonstrates how the Rust compiler infers types based on constraints given by variable declarations and usages.

The following code tells the compiler to copy into a certain generic container without the code ever explicitly specifying the contained type, using `_` as a placeholder:

```rust,editable
fn main() {
    let mut v = Vec::new();
    v.push((10, false));
    v.push((20, true));
    println!("v: {v:?}");

    let vv = v.iter().collect::<std::collections::HashSet<_>>();
    println!("vv: {vv:?}");
}
```

[`collect`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.collect) relies on `FromIterator`, which [`HashSet`](https://doc.rust-lang.org/std/iter/trait.FromIterator.html) implements.

</details>
