# How to Contribute

We'd love to accept your patches and contributions to this project. There are
just a few small guidelines you need to follow.

## Formatting

Please ensure that your files are formatted consistently. We use [`dprint`] for
this and you should follow their installation instructions for your platform. We
rely on a few tools in addition to `dprint`:

- [`rustfmt`] for formatting Rust code.
- [`yapf`] for formatting Python code.
- [`msgcat`] for formatting PO files.

### Linux

On Debian, you install `rustfmt` via `rustup` and you can install the other
tools using

```sh
sudo apt install yapf3 gettext
```

### MacOS

> _TODO: to be filled in by someone using a Mac._

### Windows

On Windows, you can should use [Gettext binaries for Windows].

> _TODO: fill in how to install `yapf` on Windows._

[`dprint`]: https://dprint.dev/
[`rustfmt`]: https://github.com/rust-lang/rustfmt
[`yapf`]: https://github.com/google/yapf
[`msgcat`]: https://www.gnu.org/software/gettext/manual/html_node/msgcat-Invocation.html
[Gettext binaries for Windows]: https://mlocati.github.io/articles/gettext-iconv-windows.html

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
