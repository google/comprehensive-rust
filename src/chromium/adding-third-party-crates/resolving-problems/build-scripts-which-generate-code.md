# Build Scripts Which Generate Code

If `ninja` complains about missing files, check the `build.rs` to see if it
writes source code files.

If so, modify [`gnrt_config.toml`][1] to add `build-script-outputs` to the
crate. If this is a transitive dependency, that is, one on which Chromium code
should not directly depend, also add `allow-first-party-usage=false`. There are
several examples already in that file:

```toml
[crate.unicode-linebreak]
allow-first-party-usage = false
build-script-outputs = ["tables.rs"]
```

Now rerun [`gnrt.py -- gen`][2] to regenerate `BUILD.gn` files to inform ninja
that this particular output file is input to subsequent build steps.

[1]: ../configuring-gnrt-config-toml.md
[2]: ../generating-gn-build-rules.md
