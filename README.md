# Comprehensive Rust 🦀

[![Build workflow](https://img.shields.io/github/actions/workflow/status/google/comprehensive-rust/build.yml?style=flat-square)](https://github.com/google/comprehensive-rust/actions/workflows/build.yml?query=branch%3Amain)
[![GitHub contributors](https://img.shields.io/github/contributors/google/comprehensive-rust?style=flat-square)](https://github.com/google/comprehensive-rust/graphs/contributors)
[![GitHub stars](https://img.shields.io/github/stars/google/comprehensive-rust?style=flat-square)](https://github.com/google/comprehensive-rust/stargazers)

This repository has the source code for Comprehensive Rust 🦀, a multi-day Rust
course developed by the Android team. The course covers all aspects of Rust,
from basic syntax to generics and error handling. It also includes deep dives on
[Android], [Chromium], [bare-metal], and [concurrency].

[Android]: https://google.github.io/comprehensive-rust/android.html
[Chromium]: https://google.github.io/comprehensive-rust/chromium.html
[bare-metal]: https://google.github.io/comprehensive-rust/bare-metal.html
[concurrency]: https://google.github.io/comprehensive-rust/concurrency.html

Read the course at **https://google.github.io/comprehensive-rust/**.

## Course Format and Target Audience

The course is used internally at Google to teach Rust to experienced software
engineers, typically with a background in C++ or Java.

The course is taught in a classroom setting, and we hope it will be useful for
others who want to teach Rust to their team. The course is less ideal for
self-study, since you would miss out on classroom discussions. You would not see
the questions and answers, nor the compiler errors we trigger when going through
the code samples. We hope to improve the self-study experience via
[speaker notes](https://github.com/google/comprehensive-rust/issues/53) and by
[publishing videos](https://github.com/google/comprehensive-rust/issues/52).

## Press

Articles and blog posts from around the web which cover Comprehensive Rust:

- 2023-09-08:
  _[Teaching Rust in 5 days](https://mo8it.com/blog/teaching-rust/)_.
  Comprehensive Rust was used as a base for a 5-day university class on Rust.
- 2023-09-21:
  _[Scaling Rust Adoption Through Training](https://security.googleblog.com/2023/09/scaling-rust-adoption-through-training.html)_.
  We published a blog post with details on the development of the course.
- 2023-10-02:
  _[In Search of Rust Developers, Companies Turn to In-House Training](https://www.darkreading.com/application-security/seeking-rust-developers-in-house-training)_.
  About how Microsoft, Google, and others are training people in Rust.
- 2024-10-18:
  _[Rust Training at Scale | Rust Global @ RustConf 2024](https://youtu.be/7h5KyMqt2-Q?si=4M99HdWWxMaqN8Zr)_.
  What Google learned from teaching Comprehensive Rust for more than two years.

## Setup

The course is built using a few tools:

- [mdbook](https://github.com/rust-lang/mdBook)
- [mdbook-svgbob](https://github.com/boozook/mdbook-svgbob)
- [mdbook-i18n-helpers and i18n-report](https://github.com/google/mdbook-i18n-helpers)
- [mdbook-exerciser](mdbook-exerciser/)
- [mdbook-course](mdbook-course/)
- [mdbook-linkcheck2](https://github.com/marxin/mdbook-linkcheck2)

First, install Rust by following the instructions on https://rustup.rs/. Then
clone this repository:

```shell
git clone https://github.com/google/comprehensive-rust/
cd comprehensive-rust
```

Then run the following command to install the correct versions of all tools
mentioned above:

```shell
cargo xtask install-tools
```

This uses `cargo install` to install the tools, so you will find them in your
`~/.cargo/bin/` directory afterwards.

## Commands

Here are some of the commonly used commands you can run in the project. Run
`cargo xtask` to view all available commands.

| Command                     | Description                                                                                                                                                                                                                                                                                                                                                                                                                            |
| --------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `cargo xtask install-tools` | Install all the tools the project depends on.                                                                                                                                                                                                                                                                                                                                                                                          |
| `cargo xtask serve`         | Start a web server with the course. You'll find the content on http://localhost:3000. To serve any of the translated versions of the course, add the language flag (--language or -l) followed by xx, where xx is the ISO 639 language code (e.g. cargo xtask serve -l da for the Danish translation).                                                                                                                                 |
| `cargo xtask rust-tests`    | Test the included Rust snippets.                                                                                                                                                                                                                                                                                                                                                                                                       |
| `cargo xtask web-tests`     | Run the web driver tests in the tests directory.                                                                                                                                                                                                                                                                                                                                                                                       |
| `cargo xtask build`         | Create a static version of the course in the `book/` directory. Note that you have to separately build and zip exercises and add them to book/html. To build any of the translated versions of the course, add the language flag (--language or -l) followed by xx, where xx is the ISO 639 language code (e.g. cargo xtask build -l da for the Danish translation). [TRANSLATIONS.md](TRANSLATIONS.md) contains further instructions. |

> **Note** On Windows, you need to enable symlinks
> (`git config --global core.symlinks true`) and Developer Mode.

## Contributing

We welcome contributions. Please see [CONTRIBUTING.md](CONTRIBUTING.md) for
details.

## Contact

For questions or comments, please contact
[Martin Geisler](mailto:mgeisler@google.com) or start a
[discussion on GitHub](https://github.com/google/comprehensive-rust/discussions).
We would love to hear from you.
