# Shared Enums

```rust,ignore
{{#include ../../../../third_party/cxx/book/snippets.rs:shared_enums_bridge}}
```

Generated Rust:

```rust
{{#include ../../../../third_party/cxx/book/snippets.rs:shared_enums_rust}}
```

Generated C++:

```c++
{{#include ../../../../third_party/cxx/book/snippets.cc:shared_enums_cpp}}
```

<details>

- On the Rust side, the code generated for shared enums is actually a struct
  wrapping a numeric value. This is because it is not UB in C++ for an enum
  class to hold a value different from all of the listed variants, and our Rust
  representation needs to have the same behavior.

</details>
