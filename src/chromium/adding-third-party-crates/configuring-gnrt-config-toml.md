# Configuring `gnrt_config.toml`

Alongside `Cargo.toml` is [`gnrt_config.toml`][0]. This contains
Chromium-specific extensions to crate handling.

If you add a new crate, you should specify at least the `group`. This is one of:

```toml
#   'safe': The library satisfies the rule-of-2 and can be used in any process.
#   'sandbox': The library does not satisfy the rule-of-2 and must be used in
#              a sandboxed process such as the renderer or a utility process.
#   'test': The library is only used in tests.
```

For instance,

```toml
[crate.my-new-crate]
group = 'test' # only used in test code
```

Depending on the crate source code layout, you may also need to use this file to
specify where its `LICENSE` file(s) can be found.

Later, we'll see some other things you will need to configure in this file to
resolve problems.

[0]: https://source.chromium.org/chromium/chromium/src/+/main:third_party/rust/chromium_crates_io/gnrt_config.toml
