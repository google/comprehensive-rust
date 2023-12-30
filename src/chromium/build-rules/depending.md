# Depending on Rust Code from Chromium C++

Simply add the above target to the `deps` of some Chromium C++ target.

```gn
import("//build/rust/rust_static_library.gni")

rust_static_library("my_rust_lib") {
  crate_root = "lib.rs"
  sources = [ "lib.rs" ]
}

# or source_set, static_library etc.
component("preexisting_cpp") {
  deps = [ ":my_rust_lib" ]
}
```

<details>
We'll see that this relationship only works if the Rust code exposes plain C APIs
which can be called from C++, or if we use a C++/Rust interop tool.
</details>
