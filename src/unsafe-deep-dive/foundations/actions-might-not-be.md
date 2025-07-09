# ... but actions on them might not be

```rust
fn main() {
    let n: i64 = 12345;
    let safe = &n as *const _;
    println!("{safe:p}");
}
```

<details>

Modify the example to de-reference `safe` without an `unsafe` block.

</details>
