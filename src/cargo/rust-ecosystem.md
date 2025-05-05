# The Rust Ecosystem

The Rust ecosystem consists of a number of tools, of which the main ones are:

- `rustc`: the Rust compiler which turns `.rs` files into binaries and other
  intermediate formats.

- `cargo`: the Rust dependency manager and build tool. Cargo knows how to
  download dependencies, usually hosted on <https://crates.io>, and it will pass
  them to `rustc` when building your project. Cargo also comes with a built-in
  test runner which is used to execute unit tests.

- `rustup`: the Rust toolchain installer and updater. This tool is used to
  install and update `rustc` and `cargo` when new versions of Rust are released.
  In addition, `rustup` can also download documentation for the standard
  library. You can have multiple versions of Rust installed at once and `rustup`
  will let you switch between them as needed.

<details>

Key points:

- Rust has a rapid release schedule with a new release coming out every six
  weeks. New releases maintain backwards compatibility with old releases ---
  plus they enable new functionality.

- There are three release channels: "stable", "beta", and "nightly".

- New features are being tested on "nightly", "beta" is what becomes "stable"
  every six weeks.

- Dependencies can also be resolved from alternative [registries], git, folders,
  and more.

- Rust also has [editions]: the current edition is Rust 2024. Previous editions
  were Rust 2015, Rust 2018 and Rust 2021.

  - The editions are allowed to make backwards incompatible changes to the
    language.

  - To prevent breaking code, editions are opt-in: you select the edition for
    your crate via the `Cargo.toml` file.

  - To avoid splitting the ecosystem, Rust compilers can mix code written for
    different editions.

  - Mention that it is quite rare to ever use the compiler directly not through
    `cargo` (most users never do).

  - It might be worth alluding that Cargo itself is an extremely powerful and
    comprehensive tool. It is capable of many advanced features including but
    not limited to:
    - Project/package structure
    - [workspaces]
    - Dev Dependencies and Runtime Dependency management/caching
    - [build scripting]
    - [global installation]
    - It is also extensible with sub command plugins as well (such as
      [cargo clippy]).
  - Read more from the [official Cargo Book]

[editions]: https://doc.rust-lang.org/edition-guide/
[workspaces]: https://doc.rust-lang.org/cargo/reference/workspaces.html
[build scripting]: https://doc.rust-lang.org/cargo/reference/build-scripts.html
[global installation]: https://doc.rust-lang.org/cargo/commands/cargo-install.html
[cargo clippy]: https://github.com/rust-lang/rust-clippy
[official Cargo Book]: https://doc.rust-lang.org/cargo/
[registries]: https://doc.rust-lang.org/cargo/reference/registries.html

</details>
