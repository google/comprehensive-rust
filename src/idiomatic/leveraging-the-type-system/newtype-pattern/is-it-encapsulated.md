---
minutes: 5
---

# Is It Truly Encapsulated?

You must evaluate the entire API surface exposed by a newtype to determine if
invariants are indeed bullet-proof. It is crucial to consider all possible
interactions, including trait implementations, that may allow users to bypass
validation checks.

```rust
pub struct Username(String);

impl Username {
    pub fn new(username: String) -> Result<Self, InvalidUsername> {
        // Validation checks...
        Ok(Self(username))
    }
}

impl DerefMut for Username {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
# impl Deref for Username {
#     type Target = str;
#
#     fn deref(&self) -> &Self::Target {
#         &self.0
#     }
# }
# pub struct InvalidUsername;
```

<details>

- `DerefMut` allows users to get a mutable reference to the wrapped value.

  The mutable reference can be used to modify the underlying data in ways that
  may violate the invariants enforced by `Username::new`!

- When auditing the API surface of a newtype, you can narrow down the review
  scope to methods and traits that provide mutable access to the underlying
  data.

- Remind students of privacy boundaries.

  In particular, functions and methods defined in the same module of the newtype
  can access its underlying data directly. If possible, move the newtype
  definition to its own separate module to reduce the scope of the audit.

</details>
