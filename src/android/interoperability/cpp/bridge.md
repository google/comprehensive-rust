# The Bridge Module

CXX relies on a description of the function signatures that will be exposed from
each language to the other. You provide this description using extern blocks in
a Rust module annotated with the `#[cxx::bridge]` attribute macro.

```rust
{{#include blobstore/src/main.rs:bridge}}
```

<details>

* The bridge is generally declared in an `ffi` module within your crate.

</details>
