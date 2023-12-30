# Exercise

Add [uwuify][0] to Chromium, turning off the crate's [default features][1].
Assume that the crate will be used in shipping Chromium, but won't be used to
handle untrustworthy input.

(In the next exercise we'll use uwuify from Chromium, but feel free to skip
ahead and do that now if you like. Or, you could create a new
[`rust_executable` target][2] which uses `uwuify`).

<details>

Students will need to download lots of transitive dependencies.

The total crates needed are:

- `instant`,
- `lock_api`,
- `parking_lot`,
- `parking_lot_core`,
- `redox_syscall`,
- `scopeguard`,
- `smallvec`, and
- `uwuify`.

If students are downloading even more than that, they probably forgot to turn
off the default features.

Thanks to [Daniel Liu][3] for this crate!

</details>

[0]: https://crates.io/crates/uwuify
[1]: https://doc.rust-lang.org/cargo/reference/features.html#the-default-feature
[2]: https://source.chromium.org/chromium/chromium/src/+/main:build/rust/rust_executable.gni
[3]: https://github.com/Daniel-Liu-c0deb0t
