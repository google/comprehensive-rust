---
minutes: 5
---

# Meaningful Doc Comments

```rust,compile_fail
/// API for the client // ❌ Lacks detail
pub mod client {}

/// Function from A to B // ❌ Redundant
fn a_to_b(a: A) -> B {...}
 
/// Connects to the database. // ❌ Lacks detail
fn connect() -> Result<(), Error> {...}
```

<details>

- Doc comments are the most common form of documentation developers engage with.

- Good doc comments provide information that the code, names, and types cannot,
  without restating the obvious information.

</details>
