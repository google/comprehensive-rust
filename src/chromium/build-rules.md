# Build rules

Rust code is usually built using `cargo`.

Chromium builds with `gn` and `ninja` for efficiency - its static rules allow
maximum parallelism. Rust is no exception.

# Adding Rust code to Chromium

In some existing Chromium .gn file, declare a `rust_static_library`:

```gn
import("//build/rust/rust_static_library.gni")

rust_static_library("my_rust_lib") {
  crate_root = "lib.rs"
  sources = [ "lib.rs" ]
}
```

You must specify _both_ the crate root, _and_ a full list of sources.
The `crate_root` is the file given to the Rust compiler representing the root
file of the compilation unit - typically `lib.rs`. `sources` is a complete
list of all source files which `ninja` needs in order to determine when rebuilds
are necessary.

You can also add `deps` on other Rust targets. Later we'll use this to
depend upon third party code.

(There's no such thing as a Rust `source_set`, because in Rust, an entire
crate is a compilation unit. A `static_library` is the smallest unit.)

# Including `unsafe` Rust code

Unsafe Rust code is forbidden in `rust_static_library` by default - it won't
compile. If you need unsafe Rust code, add `allow_unsafe = true` to the
gn target. (Later in the course we'll see circumstances why this is necessary.)

# Depending on Rust code from Chromium

Simply add the above target to the `deps` of some Chromium C++ target.

# Visual Studio Code

Types are elided in Rust code, which makes a good IDE even more useful than
for C++. Visual Studio code works well for Rust in Chromium. To use it,

* Ensure your VSCode has the `rust-analyzer` extension, not earlier forms
  of Rust support
* `gn gen out/Release --export-rust-project` (or equivalent for your output
  directory)
* `ln -s out/Release/rust-project.json rust-project.json`

We'll now try all this in an exercise.

<details>
Students might be wondering why we need a gn template, rather than using
[gn's built-in support for Rust static libraries][0].
The answer is that this template provides support for cxx interop, Rust features,
and unit tests, some of which we'll use later.
</details>

[0]: https://gn.googlesource.com/gn/+/main/docs/reference.md#func_static_library