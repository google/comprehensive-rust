---
course: Unsafe Deep Dive
session: Unsafe Deep Dive
target_minutes: 600
---

> IMPORTANT: THIS MODULE IS IN AN EARLY STAGE OF DEVELOPMENT
>
> Please do not consider this module of Comprehensive Rust to be complete. With
> that in mind, your feedback, comments, and especially your concerns, are very
> welcome.
>
> To comment on this module's development, please use the
> [GitHub issue tracker].

[GitHub issue tracker]: https://github.com/google/comprehensive-rust/issues

# Welcome to the Unsafe Rust Deep Dive

This deep dive aims to enable you to work productively with Unsafe Rust.

We’ll work on three areas:

- establishing a mental model of Unsafe Rust
- practicing reading & writing Unsafe Rust
- practicing code review for Unsafe Rust

## Links to other sections of the course

The `unsafe` keyword has treatment in:

- _Rust Fundamentals_, the main module of Comprehensive Rust, includes a session
  on [Unsafe Rust] in its last day.
- _Rust in Chromium_ discusses how to [interoperate with C++]. Consult that
  material if you are looking into FFI.
- _Bare Metal Rust_ uses unsafe heavily to interact with the underlying host,
  among other things.

[interoperate with C++]: ../chromium/interoperability-with-cpp.md
[Unsafe Rust]: ../unsafe-rust.html

<details>

The goal of this class is to teach you enough Unsafe Rust for you to be able to
review easy cases yourself, and distinguish difficult cases that need to be
reviewed my more experienced Unsafe Rust engineers.

- Establishing a mental model of Unsafe Rust
  - what the `unsafe` keyword means
  - a shared vocabulary for talking about safety
  - a mental model of how memory works
  - common patterns
  - expectations for code that uses `unsafe`

- Practice working with unsafe
  - reading and writing both code and documentation
  - use unsafe APIs
  - design and implement them

- Review code
  - the confidence to self-review easy cases
  - the knowledge to detect difficult cases

“We'll be using a spiral model of teaching. This means that we revisit the same
topic multiple times with increasing depth.”

</details>
