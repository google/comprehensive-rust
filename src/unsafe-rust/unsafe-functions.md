---
minutes: 15
---

# Unsafe Functions

A function or method can be marked `unsafe` if it has extra preconditions you
must uphold to avoid undefined behaviour.

There are two main categories:

- Rust functions declared unsafe with `unsafe fn`.
- Foreign functions in `extern "C"` blocks.

<details>

We will look at the two kinds of unsafe functions next.

</details>
