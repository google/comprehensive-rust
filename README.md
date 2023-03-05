# Comprehensive Rust 🦀

This repository has the source code for Comprehensive Rust 🦀, a four day Rust
course developed by the Android team. The course covers all aspects of Rust,
from basic syntax to generics and error handling. It also includes
Android-specific content on the last day.

Read the course at **https://google.github.io/comprehensive-rust/**.

## Course Format and Target Audience

The course is used internally at Google when teaching Rust to experienced
software engineers. They typically have a background in C++ or Java.

The course is taught in a classroom setting and we hope it will be useful for
others who want to teach Rust to their team. The course will be less useful for
self-study since you miss out on the discussions happening in the classroom. You
don't see the questions and answers and you don't see the compiler errors we
trigger when going through the code samples. We hope to improve on this via
[speaker notes](https://github.com/google/comprehensive-rust/issues/53) and by
[publishing videos](https://github.com/google/comprehensive-rust/issues/52).

## Building

The course is built using a few tools:
- [mdBook](https://github.com/rust-lang/mdBook)
- [Svgbob plugin](https://github.com/boozook/mdbook-svgbob)
- [i18n-helpers](TRANSLATIONS.md#i18n-helpers)

Install these tools with

```shell
$ cargo install mdbook
$ cargo install mdbook-svgbob
$ cargo install --path i18n-helpers
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
