# Documentation Comments

All language items in Rust can be documented using special `///` syntax.

```rust,editable
/// Determine whether the first argument is divisible by the second argument.
///
/// If the second argument is zero, the result is false.
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;  // Corner case, early return
    }
    lhs % rhs == 0     // The last expression in a block is the return value
}
```

The contents are treated as Markdown. All published Rust code is automatically
documented at `docs.rs`. It is idiomatic to document all public items in an API
using this pattern.

<details>

* Show students the docs for the `rand` crate at `docs.rs/rand`.

* Inner doc comments are discussed later and need not be addressed here.

</details>
