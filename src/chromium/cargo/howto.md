# How to minimize risk when using `cargo`

Chromium contains a copy of `cargo` and `rustc` and of many Rust crates.
If you want to stick to using them, without downloading anything from the
internet, do this:

```sh
export PATH_TO_CHROMIUM_SRC=~/chromium/src
mkdir my-rust-tool
cd my-rust-tool
mkdir .cargo
cat <<END > .cargo/config.toml
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "$PATH_TO_CHROMIUM_SRC/third_party/rust/chromium_crates_io/vendor"
END
$PATH_TO_CHROMIUM_SRC/third_party/rust-toolchain/bin/cargo init --offline
$PATH_TO_CHROMIUM_SRC/third_party/rust-toolchain/bin/cargo run --offline
```

This uses Chromium's own Rust toolchain, and Chromium's local repository
of crates which have already been through code review.

Many `cargo` commands work fine with this arrangement, but `cargo add` does
not. Nevertheless you can add add crates, so long as they're crates
we're already using for Chromium, by adding them directly to your
`Cargo.toml`:

```toml
[dependencies]
log = "0.4"
```

<details>
Bring conversation back to the previous discussion - within a given team
perhaps it's OK to use crates from the internet, allowing access to a much
wider range of crates.
</details>