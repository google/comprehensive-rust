# C++ Error Handling

```rust,ignore
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("example/include/example.h");
        fn fallible(depth: usize) -> Result<String>;
    }
}

fn main() {
    if let Err(err) = ffi::fallible(99) {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}
```

<details>

* C++ functions declared to return a `Result` will catch any thrown exception on
  the C++ side and return it as an `Err` value to the calling Rust function.
* If an exception is thrown from an extern "C++" function that is not declared
  by the CXX bridge to return `Result`, the program calls C++'s
  `std::terminate`. The behavior is equivalent to the same exception being
  thrown through a `noexcept` C++ function.

</details>
