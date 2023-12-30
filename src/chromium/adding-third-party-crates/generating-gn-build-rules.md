# Generating `gn` Build Rules

Once you've downloaded the crate, generate the `BUILD.gn` files like this:

```shell
vpython3 tools/crates/run_gnrt.py -- gen
```

Now run `git status`. You should find:

- At least one new crate source code in
  `third_party/rust/chromium_crates_io/vendor`
- At least one new `BUILD.gn` in
  `third_party/rust/<crate name>/v<major semver version>`
- An appropriate `README.chromium`

The "major semver version" is a [Rust "semver" version number][0].

Take a close look, especially at the things generated in `third_party/rust`.

<details>

Talk a little about semver --- and specifically the way that in Chromium it's to
allow multiple incompatible versions of a crate, which is discouraged but
sometimes necessary in the Cargo ecosystem.

</detail>

[0]: https://doc.rust-lang.org/cargo/reference/semver.html
