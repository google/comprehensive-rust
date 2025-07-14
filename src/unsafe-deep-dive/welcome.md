---
course: Unsafe
session: Day 1 Morning
target_minutes: 300
---

# Welcome to Unsafe Rust

> IMPORTANT: THIS MODULE IS IN AN EARLY STAGE OF DEVELOPMENT
>
> Please do not consider this module of Comprehensive Rust to be complete. With
> that in mind, your feedback, comments, and especially your concerns, are very
> welcome.
>
> To comment on this module's development, please use the
> [GitHub issue tracker].

[GitHub issue tracker]: https://github.com/google/comprehensive-rust/issues

The `unsafe` keyword is easy to type, but difficult to master. When used
appropriately, it forms a useful and indeed essential part of the Rust
programming language.

By the end of this deep dive, you'll know how to work with `unsafe` code, review
others' changes that include the `unsafe` keyword, and produce your own.

What you'll learn:

- What the terms undefined behavior, soundness, and safety mean
- Why the `unsafe` keyword exists in the Rust language
- How to write your own code using `unsafe` safely
- How to review `unsafe` code

## Links to other sections of the course

The `unsafe` keyword has treatment

- _Rust Fundamentals_, the main module of Comprehensive Rust, includes a session
  on [Unsafe Rust] in its last day.
- _Rust in Chromium_ discusses how to [interoperate with C++]. Consult that
  material if you are looking into FFI.
- _Bare Metal Rust_ uses unsafe heavily to interact with the underlying host,
  among other things.

[interoperate with C++]: ../chromium/interoperability-with-cpp.md
[Unsafe Rust]: ../unsafe-rust.html
