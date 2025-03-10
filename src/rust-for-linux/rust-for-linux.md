---
minutes: 4
---

# Getting Rust for Linux

First, we want a checkout of Linux with Rust support. The basics have been
upstream since

Then, we can follow the instructions from the Rust for Linux
[quick-start guide](https://github.com/Rust-for-Linux/linux/blob/rust/Documentation/rust/quick-start.rst#rust-analyzer):

1. Install a Rust toolchain, including standard library sources:

```sh
$ rustup override set $(scripts/min-tool-version.sh rustc)
$ rustup component add rust-src rustfmt clippy
```

This installs the oldest version of the Rust compiler supported by Rust for
Linux. Any changes should also be tested against the latest stable rustc
release.

2. Install bindgen:

```sh
$ cargo install --locked --version $(scripts/min-tool-version.sh bindgen) bindgen
```

2. Enable `CONFIG_RUST` in your kernel build configuration.

3. When building the kernel, use an LLVM toolchain:

```sh
$ make LLVM=1
```
