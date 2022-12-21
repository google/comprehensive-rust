# Slices

A slice gives you a view into a larger collection:

```rust,editable
fn main() {
    let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..4];
    println!("s: {s:?}");
}
```

* Slices borrow data from the sliced type.
* Question: What happens if you modify `a[3]`?
