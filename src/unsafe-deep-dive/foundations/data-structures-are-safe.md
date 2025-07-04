# Data structures are safe ...

Data structures are inert. They cannot do any harm by themselves.

It's possible to create a valid program with raw pointer entirely in safe code:

```rust
fn main() {
    let n: i64 = 12345;
    let safe = &n as *const _;
    println!("{safe:p}");
}
```

<details>

Consider a raw pointer to an integer, i.e. the value `safe` is the raw pointer
type `*const i64`. Raw pointers can be out-of-bounds, misaligned, or refer to
null. But the unsafe keyword is not required when creating them.

</details>
