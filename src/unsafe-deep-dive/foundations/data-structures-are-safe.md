# Data structures are safe

Data structures are inert. They cannot do any harm by themselves.

It's possible to create a valid program with raw pointer entirely in safe code:

```rust
fn main() {
    let n: i64 = 12345;
    let safe = &n as *const _;
    println!("{safe:p}");
}
```

However, using them in an unsafe way.

Consider a raw pointer to an integer, i.e. `*const i64`.
