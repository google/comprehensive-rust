---
minutes: 3
---

# Reference Validity

Rust enforces a number of rules for references that make them always safe to
use, the simplest of which is that a reference can't outlive the data it points
to.

<!-- mdbook-xgettext: skip -->

```rust,editable,compile_fail
fn main() {
    let x_ref = {
        let x = 10;
        &x
    };
    println!("x: {x_ref}");
}
```

<details>

- This slide gets students thinking about references as not simply being
  pointers, since Rust has different rules for references than other languages.

- We'll look at the rest of Rust's borrowing rules on day 3 when we talk about
  Rust's ownership system.

</details>
