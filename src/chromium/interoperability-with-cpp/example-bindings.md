# Example bindings

cxx requires you to declare the whole C++/Rust boundary in one of your `.rs`
files. For instance:

```rust,ignore
{{#include ../../../third_party/cxx/book/snippets.rs:cxx_overview}}
```

<details>
Point out:

* Native support for C++'s `std::unique_ptr` in Rust
* Native support for Rust slices in C++
* Calls from C++ to Rust, and Rust types (in the top part)
* Calls from Rust to C++, and C++ types (in the bottom part)
* If the function definitions in C++ or Rust don't match the cxx::bridge,
  a compilation failure results.

**Common misconception**: It _looks_ like a C++ header is being parser by Rust,
but this is misleading. This header is never interpreted by Rust, but simply
`#include`d in the generated C++ code for the benefit of C++ compilers.
</details>