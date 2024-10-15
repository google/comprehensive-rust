---
minutes: 1
---

# Unsafe Functions

A function or method can be marked `unsafe` if it has extra preconditions you
must uphold to avoid undefined behaviour.

There are two main categories:

- Foreign functions in `extern "C"` blocks.
- Rust functions declared unsafe with `unsafe fn`.

<details>

We will look at the two kinds of unsafe functions next.

</details>
