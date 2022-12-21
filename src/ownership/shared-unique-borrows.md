# Shared and Unique Borrows

Rust puts constraints on the ways you can borrow values:

* You can have one or more `&T` values at any given time, _or_
* You can have exactly one `&mut T` value.

```rust,editable,compile_fail
fn main() {
    let mut a: i32 = 10;
    let b: &i32 = &a;

    {
        let c: &mut i32 = &mut a;
        *c = 20;
    }

    println!("a: {a}");
    println!("b: {b}");
}
```
