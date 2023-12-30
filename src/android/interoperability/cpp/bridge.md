# The Bridge Module

CXX relies on a description of the function signatures that will be exposed from
each language to the other. You provide this description using extern blocks in
a Rust module annotated with the `#[cxx::bridge]` attribute macro.

```rust,ignore
{{#include ../../../../third_party/cxx/blobstore/src/main.rs:bridge}}
```

<details>

- The bridge is generally declared in an `ffi` module within your crate.
- From the declarations made in the bridge module, CXX will generate matching
  Rust and C++ type/function definitions in order to expose those items to both
  languages.
- To view the generated Rust code, use [cargo-expand] to view the expanded proc
  macro. For most of the examples you would use `cargo expand ::ffi` to expand
  just the `ffi` module (though this doesn't apply for Android projects).
- To view the generated C++ code, look in `target/cxxbridge`.

[cargo-expand]: https://github.com/dtolnay/cargo-expand

</details>
