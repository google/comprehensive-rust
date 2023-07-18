# Using Cargo

When you start reading about Rust, you will soon meet [Cargo](https://doc.rust-lang.org/cargo/), the standard tool
used in the Rust ecosystem to build and run Rust applications. Here we want to
give a brief overview of what Cargo is and how it fits into the wider ecosystem
and how it fits into this training.

## Installation

> **Please follow the instructions on <https://rustup.rs/>.**

This will give you the Cargo build tool (`cargo`) and the Rust compiler (`rustc`). You will also get `rustup`, a command line utility that you can use to install/switch toolchains, setup cross compilation, etc.

<details>

* On Debian/Ubuntu, you can also install Cargo, the Rust source and the [Rust formatter][6] via `apt`. However, this gets you an outdated rust version and may lead to unexpected behavior. The command would be:

```shell
    sudo apt install cargo rust-src rustfmt
```

* We suggest using [VS Code][2] to edit the code (but any LSP compatible editor works with rust-analyzer[3]).

* Some folks also like to use the [JetBrains][4] family of IDEs, which do their own analysis but have their own tradeoffs. If you prefer them, you can install the [Rust Plugin][5]. Please take note that as of January 2023 debugging only works on the CLion version of the JetBrains IDEA suite.

</details>

[2]: https://code.visualstudio.com/
[3]: https://rust-analyzer.github.io/
[4]: https://www.jetbrains.com/clion/
[5]: https://www.jetbrains.com/rust/
[6]: https://github.com/rust-lang/rustfmt
