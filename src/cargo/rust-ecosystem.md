# The Rust Ecosystem

The Rust ecosystem consists of a number of tools, of which the main ones are:

* `rustc`: the Rust compiler which turns `.rs` files into binaries and other
  intermediate formats.

* `cargo`: the Rust dependency manager and build tool. Cargo knows how to
  download dependencies hosted on <https://crates.io> and it will pass them to
  `rustc` when building your project. Cargo also comes with a built-in test
  runner which is used to execute unit tests.

* `rustup`: the Rust toolchain installer and updater. This tool is used to
  install and update `rustc` and `cargo` when new versions of Rust is released.
  In addition, `rustup` can also download documentation for the standard
  library. You can have multiple versions of Rust installed at once and `rustup`
  will let you switch between them as needed.

<details>

Key points:

* Rust has a rapid release schedule with a new release coming out
  every six weeks. New releases maintain backwards compatibility with
  old releases --- plus they enable new functionality.

* There are three release channels: "stable", "beta", and "nightly".

* New features are being tested on "nightly", "beta" is what becomes
  "stable" every six weeks.

* Rust also has [editions]: the current edition is Rust 2021. Previous
  editions were Rust 2015 and Rust 2018.

  * The editions are allowed to make backwards incompatible changes to
    the language.

  * To prevent breaking code, editions are opt-in: you select the
    edition for your crate via the `Cargo.toml` file.

  * To avoid splitting the ecosystem, Rust compilers can mix code
    written for different editions.

[editions]: https://doc.rust-lang.org/edition-guide/

</details>
