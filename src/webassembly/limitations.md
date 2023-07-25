# Limitations and Workarounds

Because it runs in a Virtual Machine with its own set of restrictions, 
some common native operations are not available in WebAssembly. For instance, 
there is no file system in WASM, therefore file operations are handled 
through Web APIs.

Other methods from the standard library that rely on system calls cannot be
used as well. They can however either be replaced by a dependency or be enabled 
by setting a feature flag. For instance,

- Many methods from `std::time` rely on system calls, so they will panic when called
in WASM. [Drop in replacements](https://docs.rs/web-time/latest/web_time/) exist that
use the brower's `Date` and `performance` methods
- `rand` which usually relies on thread local storage, can be made to use
`Math.random()` by setting the dependency feature `getrandom = { ..., features = ["js"] }`
- Statically linking against another language and bundling as one binary is mostly not supported
because the standard WASM ABI is not based on C's like in most environments. Instead, you
should dynamically link with `#[wasm_bindgen] extern "C"`. This means that some crates
which bundle C or C++ binaries will not be portable to WASM.

Furthermore, interoperability with Javascript, a language that knows nothing about 
the borrow-checker but uses Garbage Collection to manage memory forces us to
alter how we use both Rust and Javascript with WebAssembly.

This chapter covers a few caveats of WASM programming:

- [Borrow Checker](limitations/borrow-checker.md)
- [Closures](limitations/closures.md)
