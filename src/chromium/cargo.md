# Using cargo for tools

Mostly, we'll be talking about using `gn` to include Rust in Chromium itself.
But can you use `cargo` and normal Rust crates to build Rust tools and to do
Rust experiments outside the main Chromium build?

Typical Rust development requires downloading and running content from the
internet:

* The Rust toolchain (`cargo` and `rustc` at least)
* Whatever [third-party crates][0] your project depends upon. (These may
  include procedural macros and build scripts which are run on your machine
  during the build process, as well as at run time.)

Your organization's policy, and/or common sense, may prohibit you from doing
these things.

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

## Mini exercise

Discuss in small groups the policies within your own team and organization,
and come to a group agreement about what's an acceptable level of risk.

<details>
Assuming folks taking the course are physically together, ask them to discuss
in small groups of 3-4 people. Then, ask each table whether they've come
to a consensus on the level of risk

Later in the course, we'll be running an actual `cargo`-based tool, `gnrt`.
</details>

[0]: https://crates.io/