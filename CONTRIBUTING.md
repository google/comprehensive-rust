# How to Contribute

We'd love to accept your patches and contributions to this project. There are
just a few small guidelines you need to follow.

Make sure you can build the book, and that `mdbook serve` works. Please follow
the [instructions in the README].

[instructions in the README]: README.md#building

## Writing Exercises

Each segment ends with an exercise. Exercises are typically structured as an
`exercise.rs` containing the problem and solution. This is referenced from
`exercise.md` and `solution.md`, using `{{#include exercise.rs:anchor_name}}` to
match ANCHOR comments in the `exercise.rs` file. Each segment also has a
`Cargo.toml` file containing a `[[bin]]` or `[lib]` section referring to
`exercise.rs`, and that Cargo package is referenced from the workspace the root
`Cargo.toml`. The result is that `exercise.rs` is built and tested by
`cargo test`.

For segments on day 1, exercises should use `fn main() { .. }` and `dbg!` or
`println!`, with students visually verifying the correct output. On subsequent
days, prefer tests and omit `fn main() { .. }`. However, where tests would be
difficult and visual verification is more natural (such as in the Logger
exercise), using `fn main { .. }` is OK.

Especially for exercises without tests, consider including tests in
`exercise.rs` that do not appear in either `exercise.md` or `solution.md`, as
these can ensure the solution is correct.

## Testing

We test the course material in several ways:

- `mdbook test`: This will test the code samples. Some code sampes are marked
  with `ignore` in the Markdown file because the Playground is missing some of
  the crates we use. For this we have
- `cargo test`: This will build and test the Rust code found in our tooling, as
  well as the code samples which cannot be tested using the Playground.
- `npm test`: This will test the functionality of the rendered web pages. See
  the [testing README](tests/README.md) for details.

## Formatting

Please ensure that your files are formatted consistently. We use a few tools for
this:

- [`dprint`] for driving the formatting.
- [`rustfmt`] for formatting Rust code.
- [`yapf`] for formatting Python code.
- [`msgcat`] for formatting PO files.

Run `dprint fmt` to automatically format all files.

### Linux

Install `dprint` using their
[installation instructions](https://dprint.dev/install/) and install `rustfmt`
via `rustup`.

Install [pandoc 3.7.0.1](https://github.com/jgm/pandoc/releases/tag/3.7.0.1).

On Debian, you can install the other tools using:

```sh
sudo apt install yapf3 gettext texlive texlive-luatex texlive-lang-cjk texlive-lang-arabic librsvg2-bin fonts-noto
```

### MacOS

On MacOS with [Homebrew], you can install the necessary tools with:

```shell
brew install dprint yapf gettext
```

### Windows

On Windows, you can should install [Gettext tools for Windows].

Install `dprint` using their
[installation instructions](https://dprint.dev/install/) and install `rustfmt`
via `rustup`.

> _TODO: fill in how to install `yapf` on Windows._

[`dprint`]: https://dprint.dev/
[`rustfmt`]: https://github.com/rust-lang/rustfmt
[`yapf`]: https://github.com/google/yapf
[`msgcat`]: https://www.gnu.org/software/gettext/manual/html_node/msgcat-Invocation.html
[Homebrew]: https://brew.sh/
[Gettext tools for Windows]: https://github.com/vslavik/gettext-tools-windows/releases

## Contributor License Agreement

Contributions to this project must be accompanied by a Contributor License
Agreement. You (or your employer) retain the copyright to your contribution;
this simply gives us permission to use and redistribute your contributions as
part of the project. Head over to <https://cla.developers.google.com/> to see
your current agreements on file or to sign a new one.

You generally only need to submit a CLA once, so if you've already submitted one
(even if it was for a different project), you probably don't need to do it
again.

## Code Reviews

All submissions, including submissions by project members, require review. We
use GitHub pull requests for this purpose. Consult
[GitHub Help](https://help.github.com/articles/about-pull-requests/) for more
information on using pull requests.

## Community Guidelines

This project follows
[Google's Open Source Community Guidelines](https://opensource.google/conduct/).
