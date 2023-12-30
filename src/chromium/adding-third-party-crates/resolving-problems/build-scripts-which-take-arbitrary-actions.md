# Build Scripts Which Build C++ or Take Arbitrary Actions

Some crates use the [`cc`][1] crate to build and link C/C++ libraries. Other
crates parse C/C++ using [`bindgen`][2] within their build scripts. These
actions can't be supported in a Chromium context --- our gn, ninja and LLVM
build system is very specific in expressing relationships between build actions.

So, your options are:

- Avoid these crates
- Apply a patch to the crate.

Patches should be kept in
`third_party/rust/chromium_crates_io/patches/<crate>` - see for example the
[patches against the `cxx` crate][3] - and will be applied automatically by
`gnrt` each time it upgrades the crate.

[1]: https://crates.io/crates/cc
[2]: https://crates.io/crates/bindgen
[3]: https://source.chromium.org/chromium/chromium/src/+/main:third_party/rust/chromium_crates_io/patches/cxx/
