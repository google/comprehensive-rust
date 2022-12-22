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
