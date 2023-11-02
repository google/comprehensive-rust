# Adding third party crates

Rust libraries are called "crates" and are found at [crates.io][0]. It's *very
easy* for Rust crates to depend upon one another. So they do!

| Property | C++ library | Rust crate |
| --- | --- | --- |
| Build system | Lots | Consistent - `Cargo.toml` |
| Typical library size | Large | Small |
| Transitive dependencies | None | Lots |

For a Chromium engineer, this has pros and cons:

* All crates use a common build system so we can automate their inclusion into
  Chromium...
* ... but, crates typically have transitive dependencies, so you will
  likely have to bring in multiple libraries.

We'll discuss:

* How to put a crate in the Chromium source code tree
* How to make `gn` build rules for it
* How to audit its source code for sufficient safety.

[0]: https://crates.io