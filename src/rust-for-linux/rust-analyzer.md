---
minutes: 3
---

# `rust-analyzer` Setup

The `rust-analyzer` LSP server provides IDE support for working with Rust.

First, we install rust-analyzer normally:

```sh
$ rustup component add rust-analyzer
```

To use it with Rust for Linux, we need to generate a configuration file for
`rust-analyzer`[^1]:

```sh
$ make rust-analyzer
```

Then, opening our editor in the directory where the `rust-project.json` file
was created should run the language server with the appropriate settings.

[^1]: <https://github.com/Rust-for-Linux/linux/blob/rust/Documentation/rust/quick-start.rst#rust-analyzer>
