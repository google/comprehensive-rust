# Dangling References

Rust will statically forbid dangling references:

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

- A reference is said to "borrow" the value it refers to.
- Rust is tracking the lifetimes of all references to ensure they live long
  enough.
- We will talk more about borrowing when we get to ownership.
