# Exercise

Add [uwuify][0] to Chromium, turning off the crate's [default features][1].
Assume that the crate will be used in shipping Chromium, but won't be used
to handle untrustworthy input.

(In the next exercise we'll use uwuify from Chromium, but feel free to
skip ahead and do that now if you like. Or, you could create a new
[`rust_executable` target][2] which uses `uwuify`).

<details>
Students will need to download lots of transitive dependencies.

The total crates needed are: uwuify, smallvec, scopeguard, parking_lot,
parking_lot_core, lock_api and instant. If students are downloading even
more than that, they probably forgot to turn off the default features.
</detail>


[0]: https://crates.io/crates/uwuify
[1]: https://doc.rust-lang.org/cargo/reference/features.html#the-default-feature
[2]: https://source.chromium.org/chromium/chromium/src/+/main:build/rust/rust_executable.gni