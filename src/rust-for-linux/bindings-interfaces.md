---
minutes: 18
---

# Bindings and Safe Interfaces

`bindgen` is used to generate low-level, unsafe bindings for C interfaces.

But to reap the benefits of Rust, we want to use safe, foolproof interfaces to
unsafe functionality.

Subsystems are expected to implement safe interfaces on top of the low-level
generated bindings. These safe interfaces are exposed as top-level modules
within the [`kernel` crate](https://rust.docs.kernel.org/kernel/). The top-level
`bindings` module holds the unsafe `bindgen`-generated bindings, which are
generated from the C headers included by `rust/bindings/bindings_helper.h`.

In Rust for Linux, unsafe `bindgen`-generated bindings should not be used
outside the `kernel` crate. Drivers and other subsystems will make use of the
safe abstractions from this crate.

Only a subset of Linux subsystems currently have such abstractions.

It's worth browsing the
[list of modules](https://rust.docs.kernel.org/kernel/#modules) exposed by the
`kernel` crate to see what exists currently. Many of these subsystems have only
partial bindings based on the needs of consumers so far.

## Adding a Module

To add a module for some subsystem, first its header must be added to
`bindings_helper.h`. It may be necessary to write some custom code to wrap
macros or `inline` functions that are not automatically handled by `bindgen`;
this code lives in the `rust/helpers/` directory.

Then we need to write a safe abstraction using these bindings and exposing them
to the rest of kernel Rust.

Some commits from work-in-progress bindings and abstractions can provide an idea
of what it looks like to expose new kernel functionality:

- GPIO Consumer:
  [fecb4bd73f06bb2cac8e16aca7ef0e2f1b6acb50](https://github.com/Fabo/linux/commit/fecb4bd73f06bb2cac8e16aca7ef0e2f1b6acb50)
- Regmap:
  [ec0b740ac5ab299e4c86011a0002919e5bbe5c2d](https://github.com/Fabo/linux/commit/ec0b740ac5ab299e4c86011a0002919e5bbe5c2d)
- I2C:
  [70ed30fcdf8ec62fa91485c3c0a161a9d0194668](https://github.com/Fabo/linux/commit/70ed30fcdf8ec62fa91485c3c0a161a9d0194668)

## Guidelines for Abstractions

Abstractions may not be perfectly safe, but should try to be as safe as
possible. Unsafe functionality exposed should have its safety conditions
documented so that users have guidance on how to use the functionality and
justify such use.

Abstractions should also attempt to present relatively idiomatic Rust in their
interfaces:

- Follow Rust naming/capitalization conventions while remaining unsurprising to
  kernel developers.
- Use RAII instead of manual resource management where possible.
- Avoid raw pointers to bound kernel objects in favor of safer, more limited
  interfaces.

  When exposing types from generated bindings, code should make use of the
  [`Opaque<T>`](https://rust.docs.kernel.org/kernel/types/struct.Opaque.html)
  type along with native Rust references and the
  [`ARef<T>`](https://rust.docs.kernel.org/kernel/types/struct.ARef.html) type
  for types that are inherently reference-counted. This type links types'
  built-in reference count operations to the `Clone` and `Drop` traits.

## Submitting the cyclic dependency

We already know that drivers should not use unsafe bindings directly. But
subsystem maintainers may balk if they see patches submitted that add Rust
abstractions without motivation or consumers. But drivers and subsystem
abstractions may have to be submitted separately to different maintainers due to
the distributed nature of Linux development.

So how should a developer submit a driver that requires bindings/abstractions
for a subsystem not yet exposed to Rust?

There are two main approaches[^1]:

1. Submit the driver as an RFC before submitting the abstractions it relies upon
   while referencing the RFC as a potential consumer.
2. Submit a stub driver and fill out non-stub functionality as subsystem
   abstractions land.

[^1]: <https://rust-for-linux.zulipchat.com/#narrow/channel/288089-General/topic/Upstreaming.20a.20driver.20with.20unsave.20C.20API.20calls.3F/near/471677707>
