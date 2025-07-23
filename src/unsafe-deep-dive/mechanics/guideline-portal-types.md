---
minutes: 2
---

# Portal types

> TODO(timclicks): expand

Create a safe type that wraps a type that performs unsafe operations. The safe
type makes the unsafe type impossible to misuse. The wrapper acts as a portal to
the world of unsafe.

Examples:

- `std::collections::Vec` wraps `std::alloc::RawVec`
- The "sys crate" pattern
