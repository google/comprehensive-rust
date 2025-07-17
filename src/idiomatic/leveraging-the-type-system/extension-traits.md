---
minutes: 5
---

# Extension Traits

In Rust, you can't define new inherent methods for foreign types.

```rust,compile_fail
// 🛠️❌
impl &'_ str {
    pub fn is_palindrome(&self) -> bool {
        self.chars().eq(self.chars().rev())
    }
}
```

You can use the **extension trait pattern** to work around this limitation.

<details>

- Compile the example to show the compiler error that's emitted.

  Highlight how the compiler error message nudges you towards the extension
  trait pattern.

- Explain how many type-system restrictions in Rust aim to prevent _ambiguity_.

  If you were allowed to define new inherent methods on foreign types, there
  would need to be a mechanism to disambiguate between distinct inherent methods
  with the same name.

  In particular, adding a new inherent method to a library type could cause
  errors in downstream code if the name of the new method conflicts with an
  inherent method that's been defined in the consuming crate.

</details>
