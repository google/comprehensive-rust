# Configuring `third_party.toml`

Chromium has a single set of centrally-managed direct crate dependencies.
These are managed through a single [`third_party.toml` file][0] which is a lot like the
`Cargo.toml` you'd use for a cargo-native project. The main part of the file
looks like this:

```toml
[dependencies]
bitflags = "1"
cfg-if = "1"
cxx = "1"
# lots more...
```

To add a crate there, normally you'd simply add the crate and version to the
`[dependencies]` section. As with `Cargo.toml`, you can specify [more details about
the dependencies][1] - most commonly, you'll want to specify the `features` that
you wish to enable in the crate.

## What version to choose?

You should specify the [semver version][2] of the crate that you want (look it
up on [crates.io][3]!). For example, if the most recent published version of the
crate `foo` is `1.2.3``, you'd specify

```
foo = "1"
```

Under some circumstances, you'll need to put your new crate in different
areas of this file:

* If it's an executable rather than a library crate, add it to `[workspace.members]`.
* If it's a test-only dependency, add it to `[testonly-dependencies]`.

`third_party.toml` also has some [extensions][4] beyond the normal `Cargo.toml` format,
which we'll discuss later.


[0]: https://source.chromium.org/chromium/chromium/src/+/main:third_party/rust/third_party.toml
[1]: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
[2]: https://doc.rust-lang.org/cargo/reference/semver.html
[3]: https://crates.io
[4]: https://source.chromium.org/chromium/chromium/src/+/main:third_party/rust/third_party.toml;l=20