---
minutes: 10
---

# Macros

The `kernel` crate exposes some kernel functionality through macros, and
provides other macros to facilitate definitions that follow kernel patterns.

## Printing macros

The kernel provides
[`pr_info!`](https://rust.docs.kernel.org/kernel/macro.pr_info.html) and
[`dev_info!`](https://rust.docs.kernel.org/kernel/macro.dev_info.html), which
correspond to the identically-named kernel macros in C. These support string
formatting compatible with Rust's `std::print!`.

```rust
pr_info!("hello {}\n", "there");
```

## Conditional Compilation

In C, conditional compilation is done with the preprocessor using
`#if`/`#ifdef`.

Rust in the kernel may want to perform conditional compilation (which is done
with `#[cfg]` attributes in Rust) based on the same `CONFIG_FOO` macros as C
code might consider.

The kernel build system exports these as `cfg`s, so they can be used as shown
below[^1]:

```rust
#[cfg(CONFIG_X)]       // Enabled               (`y` or `m`)
#[cfg(CONFIG_X="y")]   // Enabled as a built-in (`y`)
#[cfg(CONFIG_X="m")]   // Enabled as a module   (`m`)
#[cfg(not(CONFIG_X))]  // Disabled
```

## Kernel Vtables

The kernel has slightly different requirements for its vtables than Rust traits
provide. For kernel vtables, unimplemented functions are represented by `NULL`
function pointers, while Rust traits always implement all methods.

This mismatch is resolved by providing Rust with the
[`vtable!`](https://rust-for-linux.github.io/docs/macros/attr.vtable.html)
attribute macro.

This macro is placed above trait definitions and impls and provides constant
`bool` `HAS_METHODNAME` members for each method.

## The `module!` macro

Kernel modules are defined with the `macro!` module, which we'll examine on its
own.

[^1]: <https://docs.kernel.org/rust/general-information.html#conditional-compilation>
