# How to minimize risk when using `cargo`

Chromium contains a copy of `cargo` and `rustc`

If you want to make a Rust tool, you can do something like this:

```sh
export PATH_TO_CHROMIUM_SRC=~/chromium/src
mkdir my-rust-tool
cd my-rust-tool
$PATH_TO_CHROMIUM_SRC/third_party/rust-toolchain/bin/cargo init
$PATH_TO_CHROMIUM_SRC/third_party/rust-toolchain/bin/cargo run
```

This uses Chromium's own Rust toolchain so you are not yet trusting anything
from the internet.

But, Chromium does not yet have a library of "approved" third-party crates.
Exercise due caution.
