---
minutes: 2
---

# Data structures are safe ...

Data structures are inert. They cannot do any harm by themselves.

Safe Rust code can create raw pointers:

```rust
fn main() {
    let n: i64 = 12345;
    let safe = &raw const n;
    println!("{safe:p}");
}
```

<details>

Consider a raw pointer to an integer, i.e., the value `safe` is the raw pointer
type `*const i64`. Raw pointers can be out-of-bounds, misaligned, or be null.
But the unsafe keyword is not required when creating them.

</details>
