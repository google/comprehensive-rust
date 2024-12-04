# Interoperability

Rust has excellent support for interoperability with other languages. This means
that you can:

- Call Rust functions from other languages.
- Call functions written in other languages from Rust.

When you call functions in a foreign language we say that you're using a
_foreign function interface_, also known as FFI.

<details>

- This is a key ability of Rust: compiled code becomes indistinguishable from
  compiled C or C++ code.

- Technically, we say that Rust can be compiled to the same [ABI] (application
  binary interface) as C code.

</details>

[ABI]: https://en.wikipedia.org/wiki/Application_binary_interface
