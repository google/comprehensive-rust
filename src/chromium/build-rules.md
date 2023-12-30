# Build rules

Rust code is usually built using `cargo`. Chromium builds with `gn` and `ninja`
for efficiency --- its static rules allow maximum parallelism. Rust is no
exception.

## Adding Rust code to Chromium

In some existing Chromium `BUILD.gn` file, declare a `rust_static_library`:

```gn
import("//build/rust/rust_static_library.gni")

rust_static_library("my_rust_lib") {
  crate_root = "lib.rs"
  sources = [ "lib.rs" ]
}
```

You can also add `deps` on other Rust targets. Later we'll use this to depend
upon third party code.

<details>

You must specify _both_ the crate root, _and_ a full list of sources. The
`crate_root` is the file given to the Rust compiler representing the root file
of the compilation unit --- typically `lib.rs`. `sources` is a complete list of
all source files which `ninja` needs in order to determine when rebuilds are
necessary.

(There's no such thing as a Rust `source_set`, because in Rust, an entire crate
is a compilation unit. A `static_library` is the smallest unit.)

Students might be wondering why we need a gn template, rather than using
[gn's built-in support for Rust static libraries][0]. The answer is that this
template provides support for CXX interop, Rust features, and unit tests, some
of which we'll use later.

</details>

[0]: https://gn.googlesource.com/gn/+/main/docs/reference.md#func_static_library
