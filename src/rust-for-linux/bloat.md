---
minutes: 5
---

# Avoiding Bloat

Rust for Linux makes use of `libcore` to avoid reimplementing all functionality
of the Rust standard library. But even `libcore` has some functionality built-in
that is not portable to all targets the kernel would like to support or that is
not necessary for the kernel while occupying valuable code space.

This includes[^1]:

- Support for math with 128-bit integers
- String formatting for floating-point numbers
- Unicode support for strings

Work is ongoing to make these features optional. In the meantime, the `libcore`
used by Rust for Linux is larger and less portable than it could be.

[^1]: <https://github.com/Rust-for-Linux/linux/issues/514>
