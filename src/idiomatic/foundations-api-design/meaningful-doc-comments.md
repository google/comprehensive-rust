---
minutes: 5
---

# Meaningful Doc Comments

```rust,compile_fail
/// API for Domain // ❌
pub mod domain {}

/// Function from A to B // ❌
fn a_to_b(a: A) -> B {...}
 
/// Does X // ❌
fn do_x() {}
```

Doc comments are the most common source of documentation most developers will
engage with.

It's important to write doc comments that developers will appreciate reading,
that gives them the information they are looking for and doesn't just re-state
the obvious.
