# How to Contribute

We'd love to accept your patches and contributions to this project. There are
just a few small guidelines you need to follow.

Make sure you can build the book, and that `mdbook serve` works. Please follow
the [instructions in the README].

[instructions in the README]: README.md#building

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

On Debian, you can install the other tools using:

```sh
sudo apt install yapf3 gettext
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
