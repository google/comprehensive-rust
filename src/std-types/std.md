---
minutes: 3
---

# Standard Library

Rust comes with a standard library which helps establish a set of common types
used by Rust libraries and programs. This way, two libraries can work together
smoothly because they both use the same `String` type.

In fact, Rust contains several layers of the Standard Library: `core`, `alloc`
and `std`.

- `core` includes the most basic types and functions that don't depend on
  `libc`, allocator or even the presence of an operating system.
- `alloc` includes types which require a global heap allocator, such as `Vec`,
  `Box` and `Arc`.
- Embedded Rust applications often only use `core`, and sometimes `alloc`.
