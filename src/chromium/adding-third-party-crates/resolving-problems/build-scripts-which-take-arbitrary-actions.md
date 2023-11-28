# Build scripts which build C++ or take arbitrary actions

Some crates use the [`cc`][2] crate to build and link C/C++ libraries.
Other crates parse C/C++ using [`bindgen`][3] within their build scripts.
These actions can't be supported in a Chromium context --- our gn, ninja
and LLVM build system is very specific in expressing relationships between
build actions.

So, your options are:

* Avoid these crates
* Apply a patch to the crate.

Patches should be kept in `third_party/rust/chromium_crates_io/patches/<crate>` -
see for example the [patches against the cxx crate][4]. There is currently
no automation --- [simply create and apply patches manually][5] to remove the
problematic actions from the build script.

If your patches modify the `Cargo.toml` file, rerun `gnrt gen`.

[2]: https://crates.io/crates/cc
[3]: https://crates.io/crates/bindgen
[4]: https://source.chromium.org/chromium/chromium/src/+/main:third_party/rust/chromium_crates_io/patches/cxx/
[5]: https://chromium.googlesource.com/chromium/src/+/refs/heads/main/docs/rust.md#patching-third_party-crates
