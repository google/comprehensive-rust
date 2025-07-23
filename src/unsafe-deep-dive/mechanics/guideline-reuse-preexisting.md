---
minutes: 3
---

# Reuse pre-existing code

> TODO(timclicks): expand

Avoid re-implementing:

- Interior mutability &ndash; `Cell` and `UnsafeCell`
- Wrapping NULL pointers safely &ndash; `Option<&mut T>`

<details>

When we are writing code, it can be tempting to write everything from scratch.
Check whether pre-existing solutions exist already. In particular, the standard
library offers excellent defaults for memory management.

If you find yourself writing a better implementation of these types, then
consider submitting them to the Rust project.

</details>
