---
minutes: 5
---

# Security Mitigations

Even though Rust is memory-safe, larger systems using Rust are not necessarily
memory-safe. The kernel is no exception. The kernel is often compiled with
various security mitigations and hardening flags, and to avoid undermining these
(e.g. by providing gadgets or running afoul of CPU errata), Rust code compiled
into the kernel should also be built with the same set of mitigations.

Many of these mitigations are already supported by the Rust compiler, which
merely needs to expose the same underlying LLVM functionality offered by Clang.

## Speculative execution (Meltdown/Spectre) mitigations

Recent CPU side-channel vulnerabilities in particular require changes to
compilers' code generation ("retpolines", etc.) in order to prevent userspace
access to kernel data. Support for these code-generation changes is still
pending in `rustc`[^1].

[^1]: <https://github.com/Rust-for-Linux/linux/issues/355>
