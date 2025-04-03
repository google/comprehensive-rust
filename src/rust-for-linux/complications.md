---
minutes: 5
---

# Complications and Conflicts

{{%segment outline}}

There are a number of subtleties and unresolved conflicts between the Rust
paradigm and the kernel one. These must be resolved to ship Rust code in the
kernel.

Some issues are deeper problems that require additional research and development
before Rust for Linux is ready for the prime-time; others merely require some
additional learning and attention on behalf of aspiring Rust for Linux
developers.

## 

Resolving these conflicts involves changes on both sides of the collaboration.
On the Rust side, new features land first in the Nightly edition of the compiler
before being stabilized.

To avoid waiting for stabilization, the kernel uses an
[escape hatch](https://rustc-dev-guide.rust-lang.org/building/bootstrapping/what-bootstrapping-does.html#complications-of-bootstrapping)
to access unstable features even in stable releases of the compiler. This
assists in the goal of eventually deploying Rust for Linux in Linux
distributions that ship only a stable version of the Rust toolchain.

Nonetheless, being able to build Rust for Linux using only stable Rust features
is a significant goal; the issues blocking this are tracked specifically by both
the Rust for Linux project[^1] and the Rust developers themselves[^2].

In the next slides we'll explore the most significant sources of friction
between Rust and Linux kernel development to be aware of challenges we are
likely to encounter when trying to implement kernel functionality in Rust.

[^1]: <https://github.com/Rust-for-Linux/linux/issues/2>

[^2]: <https://github.com/rust-lang/rust-project-goals/issues/116>
