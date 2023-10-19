# Rust Error Handling

```rust
#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn fallible(depth: usize) -> Result<String>;
    }
}

fn fallible(depth: usize) -> anyhow::Result<String> {
    if depth == 0 {
        return Err(anyhow::Error::msg("fallible1 requires depth > 0"));
    }
    ...
}
```

<details>

* Rust functions that return `Result` are translated to exceptions on the C++
  side.
* The exception thrown will always be of type `rust::Error`, which primarily
  exposes a way to get the error message string.
* A panic unwinding from Rust to C++ will always cause the process to
  immediately terminate.

</details>
