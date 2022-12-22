# Using Cargo

When you start reading about Rust, you will soon meet [Cargo](https://doc.rust-lang.org/cargo/), the standard tool
used in the Rust ecosystem to build and run Rust applications. Here we want to
give a brief overview of what Cargo is and how it fits into the wider ecosystem
and how it fits into this training.

On Debian/Ubuntu, you can install Cargo and the Rust source with

```shell
$ sudo apt install cargo rust-src
```

This will allow [rust-analyzer][1] to jump to the definitions. We suggest using
[VS Code][2] to edit the code (but any LSP compatible editor works).

As a sidenote: if you have the access/capability to do so, it's recommended to
install Rust's tooling via [rustup](https://rustup.rs/) since it's better integrated with the
rest of the ecosystem. As of writing, the way to install `rustup` on Unix system would be

```shell
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

If you already have the tools installed via your distro's package manager, you might want to [follow
the guide](https://rust-lang.github.io/rustup/installation/package-managers.html) so the installations
can co-exists (or just be replaced)

[1]: https://rust-analyzer.github.io/
[2]: https://code.visualstudio.com/
