# Rust Error Handling

```rust,ignore
{{#include ../../../../third_party/cxx/book/snippets.rs:rust_result}}
```

<details>

- Rust functions that return `Result` are translated to exceptions on the C++
  side.
- The exception thrown will always be of type `rust::Error`, which primarily
  exposes a way to get the error message string. The error message will come
  from the error type's `Display` impl.
- A panic unwinding from Rust to C++ will always cause the process to
  immediately terminate.

</details>
