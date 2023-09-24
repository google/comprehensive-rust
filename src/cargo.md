# Using Cargo

When you start reading about Rust, you will soon meet [Cargo](https://doc.rust-lang.org/cargo/), the standard tool
used in the Rust ecosystem to build and run Rust applications. Here we want to
give a brief overview of what Cargo is and how it fits into the wider ecosystem
and how it fits into this training.

## Installation

> **Please follow the instructions on <https://rustup.rs/>.**

This will give you the Cargo build tool (`cargo`) and the Rust compiler (`rustc`). You will also get `rustup`, a command line utility that you can use to install to different compiler versions.

After installing Rust, you should configure your editor or IDE to work with Rust. Most editors do this by talking to [rust-analyzer], which provides auto-completion and jump-to-definition functionality for [VS Code], [Emacs], [Vim/Neovim], and many others. There is also a different IDE available called [RustRover].

<details>

* On Debian/Ubuntu, you can also install Cargo, the Rust source and the [Rust formatter] via `apt`. However, this gets you an outdated rust version and may lead to unexpected behavior. The command would be:

    ```shell
    sudo apt install cargo rust-src rustfmt
    ```

</details>

[rust-analyzer]: https://rust-analyzer.github.io/
[VS Code]: https://code.visualstudio.com/
[Emacs]: https://rust-analyzer.github.io/manual.html#emacs
[Vim/Neovim]: https://rust-analyzer.github.io/manual.html#vimneovim
[RustRover]: https://www.jetbrains.com/rust/
[Rust formatter]: https://github.com/rust-lang/rustfmt
