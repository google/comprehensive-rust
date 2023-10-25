# C++ Bridge Declarations

```rust,ignore
{{#include ../../../../third_party/cxx/book/snippets.rs:cpp_bridge}}
```

<details>

* The programmer does not need to promise that the signatures they have typed in
  are accurate; that would be unreasonable. CXX performs static assertions that
  the signatures exactly correspond with what is declared in C++.
* `unsafe extern` blocks allow you to declare C++ functions that are safe to
  call from Rust.

</details>
