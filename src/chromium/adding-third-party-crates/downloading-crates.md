# Downloading Crates

A tool called `gnrt` knows how to download crates and how to generate `BUILD.gn`
rules.

To start, download the crate you want like this:

```shell
cd chromium/src
vpython3 tools/crates/run_gnrt.py -- vendor
```

> Although the `gnrt` tool is part of the Chromium source code, by running this
> command you will be downloading and running its dependencies from `crates.io`.
> See [the earlier section][0] discussing this security decision.

This `vendor` command may download:

- Your crate
- Direct and transitive dependencies
- New versions of other crates, as required by `cargo` to resolve the complete
  set of crates required by Chromium.

Chromium maintains patches for some crates, kept in
`//third_party/rust/chromium_crates_io/patches`. These will be reapplied
automatically, but if patching fails you may need to take manual action.

[0]: ../cargo.md
