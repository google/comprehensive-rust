# Using Cargo

When you start reading about Rust, you will soon meet [Cargo](https://doc.rust-lang.org/cargo/), the standard tool
used in the Rust ecosystem to build and run Rust applications. Here we want to
give a brief overview of what Cargo is and how it fits into the wider ecosystem
and how it fits into this training.

## Installation

### Rustup (Recommended)

You can follow the instructions to install cargo and rust compiler, among other standard ecosystem tools with the [rustup][3] tool, which is maintained by the Rust Foundation.

Along with cargo and rustc, Rustup will install itself as a command line utility that you can use to install/switch toolchains, setup cross compilation, etc.

### Package Managers

#### Debian

On Debian/Ubuntu, you can install Cargo, the Rust source and the [Rust formatter][6] with

```shell
sudo apt install cargo rust-src rustfmt
```

This will allow [rust-analyzer][1] to jump to the definitions. We suggest using
[VS Code][2] to edit the code (but any LSP compatible editor works).

Some folks also like to use the [JetBrains][4] family of IDEs, which do their own analysis but have their own tradeoffs. If you prefer them, you can install the [Rust Plugin][5]. Please take note that as of January 2023 debugging only works on the CLion version of the JetBrains IDEA suite.

[1]: https://rust-analyzer.github.io/
[2]: https://code.visualstudio.com/
[3]: https://rustup.rs/
[4]: https://www.jetbrains.com/clion/
[5]: https://www.jetbrains.com/rust/
[6]: https://github.com/rust-lang/rustfmt
