# Comparing Chromium and Cargo ecosystems

Rust community typically uses `cargo` and libraries from [crates.io][2].
Chromium is built using `gn` and `ninja` and a curated set of dependencies.

When writing code in Rust, your choices are:

* Use `gn` and `ninja` (using the `rust_executable` template we'll meet
  later)
* Use `cargo`, but [restrict yourself to Chromium's audited toolchain and crates][0]
* Use `cargo`, trusting a [toolchain][1] and [crates downloaded from the internet][2]

From here on we'll be focusing on `gn` and `ninja`.

## Mini exercise

Discuss in small groups the policies within your own team and
organization, and come to a group agreement about your assessment
of risk associated with the choices above.

<details>

Assuming folks taking the course are physically together, ask them to discuss
in small groups of 3-4 people. Then, ask each table whether they've come
to a consensus on the level of risk.

Later in the course, we'll be running an actual `cargo`-based tool, `gnrt`.
(`gnrt` uses Chromium's copy of `cargo` and `rustc`, but depends on third-party
libraries downloaded from the internet, although `run_gnrt.py` asks `cargo` that
only `--locked` contents are allowed via `Cargo.lock`.)

Depending on the discussion, consider also mentioning the points below:

* Talk about the cultural differences between the `cargo` world and the Chromium
  world. Point out that one reason for adopting Rust in Chromium is to tap into
  the rich, vibrant ecosystem of third-party Rust crates.
* Talk about different risk profiles of code of the Chromium browser, Chromium
  tests and tools, and other projects.  Point out that command like tools
  authored in Rust are increasingly popular across the industry --- Rust tools
  are quicker, more robust, and more ergonomic to author.
* Talk about different trust levels that one may associate with the Rust
  compiler, Rust tools (e.g. `rustup`, `cargo`), third-party tools (e.g. `cargo
  audit`), and third-party libraries (e.g. crates.io libraries with different
  popularity levels).  Point out that `rustc`, `rustup`, and `cargo` are all
  developed under the same github organization and ask if this is a valid
  argument for putting similar trust in all of these tools.
* Talk about technical differences between `gn`/`ninja` vs `cargo`:
    - Hermetic builds (e.g. using Chromium-provided `rustc` and standard library)
    - Explicitly declaring all sources, inputs, dependencies, etc.
    - Support for multiple languages (C++, Java, Rust, etc.) vs single language

</details>

[0]: https://chromium.googlesource.com/chromium/src/+/refs/heads/main/docs/rust.md#Using-cargo
[1]: https://rustup.rs/
[2]: https://crates.io/
