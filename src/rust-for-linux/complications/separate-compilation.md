---
minutes: 7
---

# Separate Compilation and Linking

One hiccup integrating Rust into the kernel compilation process is that C is
designed for full separate compilation,
where each source file can be compiled into an object file,
and then these object files are linked into a single loadable archive by the C toolchain's linker.
Rust, however, expects to compile its programs at the granularity of individual crates
and control the linking process.

In C, the compiler is not responsible for safety,
so the correctness of linking C built with different flags or compilers is left up to the user.
But compiled Rust code has no stable ABI, and so the compiler must be careful not to link together
two libraries compiled with different versions of the Rust compiler, or with different code-generation flags.

## Target modifiers

In cases where two crates are linked together, the Rust compiler will attempt to verify that they
have been compiled by the same version of the compiler to ensure that no ABI incompatibility will
undermine the memory safety of their composition.

However, if one crate was compiled with modifications to its effective ABI relative to the other
(such as forbidding usage of a register, like the `-ffixed-x18` flag does),
then it may not be valid to conclude that the resulting program will behave as intended.

The Rust compiler currently avoids this situation primarily
by treating each compiler configuration as an entirely separate target;
crates compiled for different targets may not be linked together.
But defining a fully custom target when running the compiler is a feature
only exposed by the unstable nightly version of the compiler,
which Rust for Linux does not want to commit to doing indefinitely.

The way out is a proposal[^1] to create "target modifiers",
a stable way of specifying variants of standard targets at compile-time.
Compiled crates will be stamped with the target variant
so that the Rust compiler can ensure the target modifiers match at link-time,
but users will not be required to create an entirely new compilation target.

<details>

[^1]: <https://github.com/rust-lang/rfcs/pull/3716>

</details>
