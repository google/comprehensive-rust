# Comprehensive Rust 🦀

This repository has the source code for Comprehensive Rust 🦀, a four day Rust
course developed by the Android team. The course covers all aspects of Rust,
from basic syntax to generics and error handling. It also includes
Android-specific content on the last day.

Read the course at **https://google.github.io/comprehensive-rust/**.

## Building

The course is built using [mdBook](https://github.com/rust-lang/mdBook) and its
[Svgbob plugin](https://github.com/boozook/mdbook-svgbob). Install both tools
with

```shell
$ cargo install mdbook
$ cargo install mdbook-svgbob
```

Then run

```shell
$ mdbook test
```

to test all included Rust snippets. Run

```shell
$ mdbook serve
```

to start a web server with the course. You'll find the content on
<http://localhost:3000>. You can use `mdbook build` to create a static version
of the course in the `book/` directory.

## Contact

For questions or comments, please contact [Martin
Geisler](mailto:mgeisler@google.com) or start a [discussion on
GitHub](https://github.com/google/comprehensive-rust/discussions). We would love
to hear from you.
