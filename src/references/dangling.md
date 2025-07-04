---
minutes: 3
---

# Reference Validity

Rust enforces a number of rules for references that make them always safe to
use. One rule is that references can never be `null`, making them safe to use
without `null` checks. The other rule we'll look at for now is that references
can't _outlive_ the data they point to.

```rust,editable,compile_fail
fn main() {
    let x_ref = {
        let x = 10;
        &x
    };
    dbg!(x_ref);
}
```

<details>

- This slide gets students thinking about references as not simply being
  pointers, since Rust has different rules for references than other languages.

- We'll look at the rest of Rust's borrowing rules on day 3 when we talk about
  Rust's ownership system.

## More to Explore

- Rust's equivalent of nullability is the `Option` type, which can be used to
  make any type "nullable" (not just references/pointers). We haven't yet
  introduced enums or pattern matching, though, so try not to go into too much
  detail about this here.

</details>
