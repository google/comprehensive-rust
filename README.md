# Comprehensive Rust 🦀

[![Build workflow](https://img.shields.io/github/actions/workflow/status/google/comprehensive-rust/build.yml?style=flat-square)](https://github.com/google/comprehensive-rust/actions/workflows/build.yml?query=branch%3Amain)
[![GitHub contributors](https://img.shields.io/github/contributors/google/comprehensive-rust?style=flat-square)](https://github.com/google/comprehensive-rust/graphs/contributors)
[![GitHub stars](https://img.shields.io/github/stars/google/comprehensive-rust?style=flat-square)](https://github.com/google/comprehensive-rust/stargazers)

This repository has the source code for Comprehensive Rust 🦀, a multi-day Rust
course developed by the Android team. The course covers all aspects of Rust,
from basic syntax to generics and error handling. It also includes deep dives on
[Android], [bare-metal], and [concurrency].

[Android]: https://google.github.io/comprehensive-rust/android.html
[bare-metal]: https://google.github.io/comprehensive-rust/bare-metal.html
[concurrency]: https://google.github.io/comprehensive-rust/concurrency.html

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

- [mdbook](https://github.com/rust-lang/mdBook)
- [mdbook-svgbob](https://github.com/boozook/mdbook-svgbob)
- [mdbook-i18n-helpers](https://github.com/google/mdbook-i18n-helpers)
- [mdbook-exerciser](mdbook-exerciser/)
- [mdbook-course](mdbook-course/)

First clone the repository:

```shell
git clone https://github.com/google/comprehensive-rust/
cd comprehensive-rust
```

Then install these tools with:

```shell
cargo install mdbook
cargo install mdbook-svgbob
cargo install mdbook-i18n-helpers
cargo install --path mdbook-exerciser
cargo install --path mdbook-course
```

Run

```shell
mdbook test
```

to test all included Rust snippets. Run

```shell
mdbook serve
```

to start a web server with the course. You'll find the content on
<http://localhost:3000>. You can use `mdbook build` to create a static version
of the course in the `book/` directory. Note that you have to separately build
and zip exercises and add them to `book/html`. To build any of the translated
versions of the course, run `MDBOOK_BOOK__LANGUAGE=xx mdbook build -d book/xx`
where `xx` is the ISO 639 language code (e.g. `da` for the Danish translation).
[TRANSLATIONS.md](TRANSLATIONS.md) contains further instructions.

> **Note** On Windows, you need to enable symlinks
> (`git config --global core.symlinks true`) and Developer Mode.

## Contact

For questions or comments, please contact
[Martin Geisler](mailto:mgeisler@google.com) or start a
[discussion on GitHub](https://github.com/google/comprehensive-rust/discussions).
We would love to hear from you.
