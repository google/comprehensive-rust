# Configuring the `Cargo.toml` file to add crates

Chromium has a single set of centrally-managed direct crate dependencies. These
are managed through a single [`Cargo.toml`][0]:

```toml
[dependencies]
bitflags = "1"
cfg-if = "1"
cxx = "1"
# lots more...
```

As with any other `Cargo.toml`, you can specify
[more details about the dependencies][1] --- most commonly, you'll want to
specify the `features` that you wish to enable in the crate.

When adding a crate to Chromium, you'll often need to provide some extra
information in an additional file, `gnrt_config.toml`, which we'll meet next.

[0]: https://source.chromium.org/chromium/chromium/src/+/main:third_party/rust/chromium_crates_io/Cargo.toml
[1]: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
