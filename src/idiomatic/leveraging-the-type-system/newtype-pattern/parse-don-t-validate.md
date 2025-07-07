---
minutes: 5
---

# Parse, Don't Validate

The newtype pattern can be leveraged to enforce _invariants_.

```rust
pub struct Username(String);

impl Username {
    pub fn new(username: String) -> Result<Self, InvalidUsername> {
        if username.is_empty() {
            return Err(InvalidUsername::CannotBeEmpty)
        }
        if username.len() > 32 {
            return Err(InvalidUsername::TooLong { len: username.len() })
        }
        // Other validation checks...
        Ok(Self(username))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
# pub enum InvalidUsername {
#     CannotBeEmpty,
#     TooLong { len: usize },
# }
```

<details>

- The newtype pattern, combined with Rust's module and visibility system, can be
  used to _guarantee_ that instances of a given type satisfy a set of
  invariants.

  In the example above, the raw `String` stored inside the `Username` struct
  can't be accessed directly from other modules or crates, since it's not marked
  as `pub` or `pub(in ...)`. Consumers of the `Username` type are forced to use
  the `new` method to create instances. In turn, `new` performs validation, thus
  ensuring that all instances of `Username` satisfy those checks.

- The `as_str` method allows consumers to access the raw string representation
  (e.g., to store it in a database) but, thanks to Rust's borrow checker, they
  can't modify it.

- Stress the importance of evaluating _the entire API surface_ exposed by a
  newtype to determine if invariants are indeed bullet-proof.\
  It is crucial to consider all possible interactions, including trait
  implementations, that may allow users to bypass the invariants. For example,
  if the `Username` type implements the `DerefMut` trait, users can modify the
  underlying string directly, bypassing the validation checks in `new`.

- Type-level invariants have second-order benefits.\
  The input is validated once, at the boundary, and the rest of the program can
  rely on the invariants being upheld. We can avoid redundant validation and
  "defensive programming" checks throughout the program, reducing noise and
  improving performance.

</details>
