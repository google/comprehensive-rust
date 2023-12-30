# Adding Third Party Crates

Rust libraries are called "crates" and are found at [crates.io][0]. It's _very
easy_ for Rust crates to depend upon one another. So they do!

| Property                | C++ library | Rust crate               |
| ----------------------- | ----------- | ------------------------ |
| Build system            | Lots        | Consistent: `Cargo.toml` |
| Typical library size    | Large-ish   | Small                    |
| Transitive dependencies | Few         | Lots                     |

For a Chromium engineer, this has pros and cons:

- All crates use a common build system so we can automate their inclusion into
  Chromium...
- ... but, crates typically have transitive dependencies, so you will likely
  have to bring in multiple libraries.

We'll discuss:

- How to put a crate in the Chromium source code tree
- How to make `gn` build rules for it
- How to audit its source code for sufficient safety.

[0]: https://crates.io

<details>
All of the things in the table on this slide are generalizations, and
counter-examples can be found. But in general it's important for students
to understand that most Rust code depends on other Rust libraries, because
it's easy to do so, and that this has both benefits and costs.
</details>
