# Using cargo for tools

As you'll see, Chromium builds don't use `cargo`. But typical Rust development
uses `cargo` and requires downloading and running content from the
internet:

* The Rust toolchain (`cargo` and `rustc` at least)
* Whatever [third-party crates][0] your project depends upon. (These may
  include procedural macros and build scripts which are run on your machine
  during the build process, as well as at run time.)

Your organization's policy, and/or common sense, may prohibit you from doing
these things.


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
