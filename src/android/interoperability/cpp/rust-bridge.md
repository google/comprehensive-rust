# Rust Bridge Declarations

```rust,ignore
{{#include ../../../../third_party/cxx/book/snippets.rs:rust_bridge}}
```

<details>

- Items declared in the `extern "Rust"` reference items that are in scope in the
  parent module.
- The CXX code generator uses your `extern "Rust"` section(s) to produce a C++
  header file containing the corresponding C++ declarations. The generated
  header has the same path as the Rust source file containing the bridge, except
  with a .rs.h file extension.

</details>
