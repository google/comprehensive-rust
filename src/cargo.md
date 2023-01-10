# Using Cargo

When you start reading about Rust, you will soon meet [Cargo](https://doc.rust-lang.org/cargo/), the standard tool
used in the Rust ecosystem to build and run Rust applications. Here we want to
give a brief overview of what Cargo is and how it fits into the wider ecosystem
and how it fits into this training.

## Installation

### Rustup (Recommended)

You can follow the instructions to install cargo and rust compiler, among other standard ecosystem tools with the [rustup][3] tool, which is maintained by the Rust Foundation.

On Unix-like systems you can curl down the install script and pipe it into your shell like so:

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

On windows there is a `rustup-init.exe` available on the website.

Rustup also installs itself as an additional command line utility that you can use to install toolchains, setup cross compilation, etc.

### Package Managers

#### Debian

On Debian/Ubuntu, you can install Cargo and the Rust source with

```shell
$ sudo apt install cargo rust-src
```

This will allow [rust-analyzer][1] to jump to the definitions. We suggest using
[VS Code][2] to edit the code (but any LSP compatible editor works).

[1]: https://rust-analyzer.github.io/
[2]: https://code.visualstudio.com/
[3]: https://rustup.rs/
